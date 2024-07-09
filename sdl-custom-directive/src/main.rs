use async_graphql::*;

#[TypeDirective(location = "ArgumentDefinition")]
fn testDirective(desc: String) {}

struct Query;

#[Object]
impl Query {
    async fn add(
        &self,
        #[graphql(directive = testDirective::apply("Desc for a".into()))] a: i32,
        #[graphql(directive = testDirective::apply("Desc for b".into()))] b: i32,
    ) -> i32 {
        a + b
    }
}

fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    std::fs::write("./schema.graphql", schema.sdl()).unwrap();
}
