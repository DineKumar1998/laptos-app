use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

async fn fetch_data() -> Result<Vec<Post>, reqwest::Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts").await?;
    let res = response.text().await?;
    let posts: Vec<Post> = serde_json::from_str(&res).unwrap();
    Ok(posts)
}

#[component]
pub fn App() -> impl IntoView {
    // Create a signal to hold the posts
    // let (posts, set_posts) = signal(vec![]);

    let data = LocalResource::new(move || fetch_data());

    // let res = move || {
    //     view! {
    //         <For
    //             each=move || {
    //                 if let Some(Ok(posts)) = data.read().as_deref() {
    //                     Vec::clone(&posts)
    //                 } else {
    //                     Vec::new()
    //                 }
    //             }
    //             key=move |post| post.id
    //             children=move |post| {
    //                 view!{
    //                     <div>
    //                         <h2>{post.title.clone()}</h2>
    //                         <p>{post.body.clone()}</p>
    //                     </div>
    //                 }
    //             }
    //         />
    //     }
    // };

    let res = move || {
        data.read().as_deref().map_or_else(
            || view! { <div> "Loading..." </div> }.into_any(),
            |posts| {
                if let Ok(post_view) = posts {
                    return view! {
                        <h1>"Posts Renerinng"</h1>
                        <div class="container" >
                         {post_view.iter().map(|post| { view!{
                            <div>
                                <h2>{post.title.clone()}</h2>
                                <p>{post.body.clone()}</p>
                            </div>
                         }}).collect::<Vec<_>>()}
                         </div>
                    }
                    .into_any();
                } else {
                    return view! {
                        <h1>"Posts Not Found"</h1>
                    }
                    .into_any();
                }
            },
        )
    };

    view! {
        <div>
            <h1>"Posts"</h1>
            {res}
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
