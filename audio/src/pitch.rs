use std::fmt;

use approx::AbsDiffEq;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::Hz;

#[derive(Copy, Clone, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Semitone {
    C = 0,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

enum Sign {
    Flat,
    None,
    Sharp,
}

impl Semitone {
    pub fn try_from_prefix(val: &str) -> Result<(Self, &str), String> {
        // Get the first char:
        let mut chars = val.chars();
        let letter = chars.next().ok_or("Empty note")?;
        let mut remain = chars.as_str();

        // is it sharp, flat, or not?
        let sign = if let Some(ch) = chars.next() {
            match ch {
                '♯' | '#' | 's' => {
                    remain = chars.as_str();
                    Sign::Sharp
                }
                '♭' | 'b' => {
                    remain = chars.as_str();
                    Sign::Flat
                }
                _ => Sign::None,
            }
        } else {
            Sign::None
        };

        // Which note?
        let bad_sign = Err(format!("Bad sign: {val}"));
        let semitone = match letter {
            'C' => match sign {
                Sign::Flat => bad_sign,
                Sign::None => Ok(Self::C),
                Sign::Sharp => Ok(Self::Cs),
            },
            'D' => match sign {
                Sign::Flat => Ok(Self::Cs),
                Sign::None => Ok(Self::D),
                Sign::Sharp => Ok(Self::Ds),
            },
            'E' => match sign {
                Sign::Flat => Ok(Self::Ds),
                Sign::None => Ok(Self::E),
                Sign::Sharp => bad_sign,
            },
            'F' => match sign {
                Sign::Flat => bad_sign,
                Sign::None => Ok(Self::F),
                Sign::Sharp => Ok(Self::Fs),
            },
            'G' => match sign {
                Sign::Flat => Ok(Self::Fs),
                Sign::None => Ok(Self::G),
                Sign::Sharp => Ok(Self::Gs),
            },
            'A' => match sign {
                Sign::Flat => Ok(Self::Gs),
                Sign::None => Ok(Self::A),
                Sign::Sharp => Ok(Self::As),
            },
            'B' => match sign {
                Sign::Flat => Ok(Self::As),
                Sign::None => Ok(Self::B),
                Sign::Sharp => bad_sign,
            },
            _ => Err(format!("Bad note: {val}")),
        };

        // Figure out how many characters remained
        semitone.map(|s| (s, remain))
    }
}

impl TryFrom<&str> for Semitone {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match Semitone::try_from_prefix(val) {
            Ok((semitone, remain)) => {
                if remain.is_empty() {
                    Ok(semitone)
                } else {
                    Err(format!("Trailing garbage: {val}"))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Semitone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Semitone::C => "C",
            Semitone::Cs => "C♯",
            Semitone::D => "D",
            Semitone::Ds => "D♯",
            Semitone::E => "E",
            Semitone::F => "F",
            Semitone::Fs => "F♯",
            Semitone::G => "G",
            Semitone::Gs => "G♯",
            Semitone::A => "A",
            Semitone::As => "A♯",
            Semitone::B => "B",
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pitch {
    /// e.g. the 4 in C4:
    octave: i16,
    /// Which semitone in the octave (C is 0)
    semitone: Semitone,
    /// How far out of tune:
    cents: f32,
}

impl Pitch {
    pub fn new(semitone: Semitone, octave: i16) -> Self {
        Pitch {
            semitone,
            octave,
            cents: 0.,
        }
    }

    pub fn new_with_cents(semitone: Semitone, octave: i16, cents: f32) -> Self {
        Pitch {
            semitone,
            octave,
            cents,
        }
    }
}

impl TryFrom<&str> for Pitch {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match Semitone::try_from_prefix(val) {
            Ok((semitone, remain)) => {
                // TODO: parse cents?
                match remain.parse::<i16>() {
                    Ok(octave) => Ok(Pitch {
                        octave,
                        semitone,
                        cents: 0.,
                    }),
                    Err(e) => Err(format!("Bad octave: {val}: {e}")),
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cents == 0. {
            write!(f, "{}{}", self.semitone, self.octave)
        } else {
            write!(f, "{}{}{:+}", self.semitone, self.octave, self.cents)
        }
    }
}

impl AbsDiffEq for Pitch {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.octave == other.octave && self.semitone == other.semitone {
            self.cents.abs_diff_eq(&other.cents, epsilon)
        } else {
            false
        }
    }
}

pub struct Tuning {
    ref_freq: Hz,
    ref_pitch: Pitch,
}

impl Tuning {
    pub const A440: Tuning = Tuning {
        ref_freq: Hz(440.),
        ref_pitch: Pitch {
            octave: 4,
            semitone: Semitone::A,
            cents: 0.,
        },
    };

    pub fn pitch_from(&self, freq: Hz) -> Pitch {
        // Number of semitones from the reference frequency:
        let mut semitones = (freq.0 / self.ref_freq.0).log2() * 12.0;
        // From the bottom of the reference octave:
        semitones += self.ref_pitch.semitone as u8 as f32;
        // Nearest whole semitone and distance in cents:
        let cents = (semitones - semitones.round()) * 100.0;
        let semitones = semitones.round() as i32;
        // Number of octaves from the reference, and pitch within octave:
        let octaves = semitones.div_euclid(12);
        let semitone = Semitone::from_i32(semitones.rem_euclid(12)).unwrap();
        Pitch::new_with_cents(semitone, octaves as i16 + self.ref_pitch.octave, cents)
    }

    pub fn freq_from(&self, pitch: Pitch) -> Hz {
        let mut semitones = pitch.semitone as i32;
        // Distance bottom of reference octave:
        semitones += (pitch.octave - self.ref_pitch.octave) as i32 * 12;
        // Distance from reference semitone:
        semitones -= self.ref_pitch.semitone as i32;
        let semitones = semitones as f32 + pitch.cents / 100.0;

        Hz(self.ref_freq.0 * f32::powf(2.0, semitones / 12.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semitone_from() {
        assert_eq!(Semitone::from_u8(2), Some(Semitone::D));
        assert_eq!(Semitone::try_from("F♯"), Ok(Semitone::Fs));
        assert_eq!(Semitone::try_from("G"), Ok(Semitone::G));
        assert_eq!(Semitone::try_from("G♭"), Ok(Semitone::Fs));
        assert!(Semitone::try_from("F♯4").is_err());
    }

    #[test]
    fn semitone_all_roundtrip() {
        for i in 0..12 {
            let start = Semitone::from_u8(i).unwrap();
            assert_eq!(Ok(start), Semitone::try_from(start.to_string().as_str()))
        }
    }

    #[test]
    fn pitch_from() {
        assert_eq!(Pitch::try_from("C42"), Ok(Pitch::new(Semitone::C, 42)));
        assert_eq!(Pitch::try_from("C-2"), Ok(Pitch::new(Semitone::C, -2)));
        assert_eq!(Pitch::try_from("F♯4"), Ok(Pitch::new(Semitone::Fs, 4)));
        assert_eq!(Pitch::try_from("F♯-1"), Ok(Pitch::new(Semitone::Fs, -1)));
    }

    #[test]
    fn pitch_display() {
        assert_eq!(Pitch::new(Semitone::C, 4).to_string(), "C4");
        assert_eq!(
            Pitch::new_with_cents(Semitone::C, 4, 7.).to_string(),
            "C4+7"
        );
        assert_eq!(
            Pitch::new_with_cents(Semitone::C, 4, -14.).to_string(),
            "C4-14"
        );
    }

    #[test]
    fn freq_to_pitch() {
        assert_eq!(
            Tuning::A440.pitch_from(Hz(440.)),
            Pitch::new(Semitone::A, 4)
        );
        assert_abs_diff_eq!(
            Tuning::A440.pitch_from(Hz(9.722)),
            Pitch::new(Semitone::Ds, -1),
            epsilon = 0.5
        );
        assert_abs_diff_eq!(
            Tuning::A440.pitch_from(Hz(167.)),
            Pitch::new_with_cents(Semitone::E, 3, 23.),
            epsilon = 0.5
        );
        assert_abs_diff_eq!(
            Tuning::A440.pitch_from(Hz(479.5)),
            Pitch::new_with_cents(Semitone::As, 4, 49.),
            epsilon = 0.5
        );
        assert_abs_diff_eq!(
            Tuning::A440.pitch_from(Hz(480.)),
            Pitch::new_with_cents(Semitone::B, 4, -49.),
            epsilon = 0.5
        );
        assert_abs_diff_eq!(
            Tuning::A440.pitch_from(Hz(740.)),
            Pitch::new(Semitone::Fs, 5),
            epsilon = 0.5
        );
    }

    #[test]
    fn pitch_to_freq() {
        assert_eq!(Tuning::A440.freq_from(Pitch::new(Semitone::A, 4)), Hz(440.));
        assert_abs_diff_eq!(
            Tuning::A440
                .freq_from(Pitch::new_with_cents(Semitone::B, 3, 49.))
                .0,
            254.0,
            epsilon = 0.5
        );
        assert_abs_diff_eq!(
            Tuning::A440
                .freq_from(Pitch::new_with_cents(Semitone::E, 5, -49.))
                .0,
            640.8,
            epsilon = 0.5
        );
    }
}
