#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
    data: String,
}

impl RibonucleicAcid {
    pub fn new(data: &str) -> RibonucleicAcid {
        RibonucleicAcid { data: data.to_string() };
    }

    pub fn as_ref(&self) -> String {
        self.data.clone()
    }
}

#[derive(PartialEq, Eq)]
pub struct DeoxyribonucleicAcid {
    data: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(data: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { data: data.to_string() }
    }

    fn transcribe(&self, s: char) -> char {
        match s {
            'C' => 'G',
            'G' => 'C',
            'A' => 'U',
            'T' => 'A',
            _ => s,
        }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid { data: self.data.chars().map(|c| self.transcribe(c)).collect::<String>() }
    }
}
