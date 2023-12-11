struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    collection.into_iter().filter(|item| condition.is_match(item)).collect()
}

fn main() {
    
    let numbers = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { value: 3 };

   
    let filtered_numbers = custom_filter(numbers, &filter_condition);

  
    println!("Filtered Numbers: {:?}", filtered_numbers);
}
