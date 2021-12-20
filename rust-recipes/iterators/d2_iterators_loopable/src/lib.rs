
// Recipe for receiving a large variety of different loopable items

pub struct Company {
    ceo: String,
    receptionist: String,
    marketing: String,
}

// Lifetime of the company must be longer than that of the CompanyIter
pub struct CompanyIter<'a> {
    c: &'a Company, // Ref to Company
    n: i32,
}

impl<'a> Iterator for CompanyIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option <Self::Item> {
        self.n += 1;
        match self.n {
            1 => Some(&self.c.ceo),
            2 => Some(&self.c.receptionist),
            3 => Some(&self.c.marketing),
            _ => None,
        }
    }
}

// Implement this for Company for lifetime 'a
impl<'a> IntoIterator for &'a Company {
    type IntoIter = CompanyIter<'a>;
    type Item = &'a str;

    // This consumes the object
    fn into_iter(self) -> Self::IntoIter {
        CompanyIter {  c: &self,  n: 0, }
    }
}

#[cfg(test)]
mod tests_company {
    use super::*;
    #[test]
    fn test_info_iter() {
        let c = Company{
            ceo: "Alice".to_string(),
            receptionist: "Bob".to_string(),
            marketing: "Chad".to_string(),
        };
        let mut res = String::new();
        for m in &c {
            res.push_str(m);
        }
        assert_eq!(res, "AliceBobChad");
    }
}

