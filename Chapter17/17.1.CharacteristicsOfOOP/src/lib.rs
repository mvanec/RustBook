pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        let coll = AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        };
        coll
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut coll = AveragedCollection::new();

        for index in 1..=5 {
            coll.add(index);
        }
        let result = coll.average();
        assert_eq!(result, 3.0);
    }
}
