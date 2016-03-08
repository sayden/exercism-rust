#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
    data: String,
}

impl RibonucleicAcid {
    pub fn new(data: &str) -> RibonucleicAcid {
        return RibonucleicAcid { data: data.to_string() };
    }

    pub fn as_ref(&self) -> String {
        return self.data.clone();
    }
}

#[derive(PartialEq, Eq)]
pub struct DeoxyribonucleicAcid {
    data: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(data: &str) -> DeoxyribonucleicAcid {
        return DeoxyribonucleicAcid { data: data.to_string() };
    }

    fn transcribe(&self, s: char) -> char {
        match s {
            'C' => return 'G',
            'G' => return 'C',
            'A' => return 'U', 
            'T' => return 'A',
            _ => s,
        }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        return RibonucleicAcid {
            data: self.data.chars().map(|c| self.transcribe(c)).collect::<String>(),
        };
    }
}
