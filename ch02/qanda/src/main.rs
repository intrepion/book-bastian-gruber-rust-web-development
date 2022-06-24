struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
 
struct QuestionId(String);
 
impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
 
fn main() {
    let question = Question::new("1", "First Question", "Content of question", ["faq"]);
    println!("{}", question);
}
