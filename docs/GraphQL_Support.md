IRAM uses [GraphQL](https://graphql.org/) for integrating with frontend for most of the use-cases. 

An alert has a lot of information by default and not all is required at once. Using Graphql, not only helps us reduce the flow of unnecessary fields to the frontend but also prevents the efforts of making duplicate APIs for different use cases.

Different functions are exposed for querying alerts with different states and filters.

### Why don't we have one generic function for alerts?
Taking filter options directly from client exposes too many possibilities for input and output. Hence the behaviour of the application doesn't remain predictable since there are variety of outcomes possible.
It is better to expose certain functions only for external clients. Internally, application can have a common processing function and different request processors

### How are graphql requests authenticated?
When user logins, a token is returned. For IRAM-UI, this token is stored in client's local storage and added automatically for every request made by UI.