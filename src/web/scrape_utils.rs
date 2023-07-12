use std::collections::HashSet;

use scraper::ElementRef;



/// Takes in an html element reference and returns the text from within
/// This is to be used on recipes to grab the prep time, cook time and total time
pub fn scrape_recipe_time(element: ElementRef) -> Option<String> {
    let mut target_time = String::new();
    if let Some(sibling) = element.next_sibling() {

        let temp = ElementRef::wrap(sibling).unwrap();

        let mut collected_text: Vec<&str> = temp.text().collect::<Vec<_>>();

        let mut seen: HashSet<&str> = HashSet::new();

                        
        let mut formatted_text = collected_text.iter().map(|item| item.trim())
            .collect::<Vec<_>>();

        // Removing duplicate text
        formatted_text.retain(|item| seen.insert(*item));
        

        target_time = formatted_text.join(" ").to_owned();   
        return Some(target_time)     
    }
    None
}