use crate::traits::{ num::IsNum, saturating::IsSaturating };

pub trait IsSize: IsNum + IsSaturating {}