use super::init::{Template, make_template};

/// Sets up the blog template and prints its details.
pub fn setup_blog_template() {
    println!("Using the blog template.");
    let template: Vec<Template> = vec![
        Template {
            path: "/",
            file: "index.json",
            content: r#"
{
    "blog_title": "My personal blog!",
    "about_me": "I like to eat chocolate and I want to be a businessman",
    "posts_link": "/posts"
}
"#,
        },
        Template {
            path: "/posts",
            file: "posts.json",
            content: r#"
{
    "posts": [
        {
            "id": 1,
            "name": "Bananas!",
            "description": "Bananas are very good for health.",
            "link": "/posts/1"
        },
        {
            "id": 2,
            "name": "Coffee and Java?",
            "description": "Indeed, the Java logo is a cup of coffee.",
            "link": "/posts/2"
        },
        {
            "id": 3,
            "name": "GraphQL is very fantastic",
            "description": "GraphQL is very useful when making APIs.",
            "link": "/posts/3"
        }
    ]
}
"#,
        },
        Template {
            path: "/posts/1",
            file: "posts_1.json",
            content: r#"
{
    "id": 1,
    "name": "Bananas!",
    "description": "Bananas are very good for health.",
    "content": "Bananas, scientifically known as 'Musa paradisiaca,' are one of the most popular fruits worldwide. While commonly referred to as a fruit, botanically, bananas are classified as berries. They are known to be an excellent source of potassium, vitamin C, vitamin B6, and dietary fiber. Did you know that the inside of a banana peel can be used to soothe mosquito bites or alleviate itching? Additionally, bananas contain natural sugars such as sucrose, fructose, and glucose, which provide a quick energy boost. Bananas are believed to have originated in Southeast Asia, and there are currently over 1,000 banana varieties worldwide. They are incredibly versatile and can be enjoyed on their own, used in smoothies, baked goods, and savory dishes. When fully ripe, banana peels turn yellow, but they gradually darken to brown as they ripen further. These random facts about bananas can be interesting to share in conversations or to expand your knowledge about this delicious fruit."
}
"#,
        },

        Template {
            path: "/posts/2",
            file: "posts_2.json",
            content: r#"
{
    "id": 2,
    "name": "Coffee and Java?",
    "description": "Indeed, the Java logo is a cup of coffee.",
    "content": "Java, one of the most popular programming languages in the world, has a unique and delightful connection to coffee. From its name to its community and even its role in coffee machine technology, Java and coffee are intertwined in a fascinating way. When James Gosling and his team were developing the language, they sought inspiration from the rich coffee culture. Originally named 'Oak,' they eventually renamed it 'Java' to pay homage to the coffee plantations of Indonesia. This change represented their admiration for the aromatic beverage that brings people together. The connection between Java and coffee doesn't stop at the name. As you delve into Java's documentation and libraries, you'll encounter subtle references to coffee throughout. Terminology like 'Java beans' (reminiscent of the coffee bean) and 'CyclicBarrier' (which mirrors the idea of people meeting up at a coffee shop) adds a touch of whimsy to the programming experience. Just as coffee shops serve as gathering places, Java has fostered a vibrant and collaborative community. Developers worldwide congregate in forums, social media groups, and conferences to share their knowledge, collaborate on projects, and enjoy the camaraderie that echoes the convivial atmosphere of a coffee house. Java's extensive ecosystem of libraries and frameworks ensures that developers can find the perfect blend of tools for their projects. Java's appeal extends beyond software development; it has even found a place in the world of coffee machines. Some advanced coffee machines utilize Java-based software to control various aspects of the brewing process. From temperature regulation to custom brewing profiles, Java enables coffee aficionados to enjoy their favorite cup of joe with precise control and customization. Java's philosophy of 'Write once, run anywhere' aligns with coffee's global popularity. Just as coffee transcends borders and cultures, Java allows developers to write code that can run on multiple platforms and operating systems without extensive modifications. This platform independence has played a significant role in Java's widespread adoption and its ability to cater to diverse technological environments. In the realm of technology, where Java and coffee intersect, a delightful synergy emerges. Whether you're sipping a cup of coffee while coding in Java or appreciating the clever nods to coffee within the language itself, the connection between Java and coffee adds a touch of warmth and creativity to the world of programming."
}
"#,
        },
        Template {
            path: "/posts/3",
            file: "posts_3.json",
            content: r#"
{
    "id": 3,
    "name": "GraphQL is very fantastic",
    "description": "GraphQL is very useful when making APIs.",
    "content": "GraphQL has revolutionized the way we design and consume APIs, offering a remarkable set of benefits that have propelled its widespread adoption. Here's a text highlighting the many virtues of GraphQL: GraphQL has emerged as a game-changer in the world of API development. With its flexible and efficient approach to data querying, it has transformed the way we interact with and retrieve information from our APIs. One of the key advantages of GraphQL is its ability to empower clients to request precisely the data they need. Instead of relying on predefined endpoints that return fixed data structures, GraphQL allows clients to shape their queries dynamically. This eliminates over-fetching and under-fetching of data, enabling applications to retrieve only the necessary information in a single request. This efficiency results in faster and more responsive applications, as well as reduced bandwidth usage. Furthermore, GraphQL provides a strongly typed schema that serves as a contract between the server and clients. This schema acts as a self-documenting blueprint of available data and operations, making it easier for developers to understand and explore the API capabilities. This enhanced discoverability fosters effective collaboration between frontend and backend teams, streamlining the development process and reducing the likelihood of miscommunication. Another remarkable feature of GraphQL is its support for real-time updates. By leveraging subscriptions, clients can subscribe to specific data changes and receive immediate updates whenever those changes occur on the server. This real-time capability opens up a world of possibilities for building interactive applications, such as chat systems, live dashboards, or collaborative editing tools. Moreover, GraphQL embraces a declarative approach to data fetching. Clients can express their data requirements using a straightforward and intuitive syntax, specifying the fields they want to retrieve and the relationships they want to traverse. This declarative nature promotes simplicity and reduces the complexity of managing multiple API endpoints, resulting in more maintainable and scalable codebases. Additionally, GraphQL is agnostic to any specific database or storage system, allowing developers to consolidate and unify data from multiple sources seamlessly. It serves as a layer of abstraction that shields clients from the intricacies of underlying data structures, enabling backend teams to evolve their data models without impacting clients. This flexibility is particularly advantageous in microservices architectures or environments with heterogeneous data backends. Furthermore, the GraphQL community has flourished, with a rich ecosystem of tools and libraries that make working with GraphQL even more convenient. From client-side frameworks and code generators to server-side middleware and developer tools, the GraphQL ecosystem offers a wealth of resources to enhance productivity and accelerate development. In conclusion, GraphQL represents a significant advancement in API design, offering a multitude of benefits such as efficient data retrieval, real-time updates, simplicity in querying, and seamless integration with diverse data sources. Its versatility and developer-friendly features have made it a compelling choice for modern application development, empowering teams to build performant, scalable, and flexible APIs."
}
"#,
        },
    ];

    make_template(template);
}
