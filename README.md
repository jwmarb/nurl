## Frontend

Install dependencies:

To run:

```sh
pnpm install
```

## Backend

No need to install dependencies--Rust will download and compile their binaries.

To run:

```sh
cargo run
```

# Todo list

1. Add authentication (simple JWT workflow)

- user database must follow the following schema:

  ```ts
  type Schema = {
    id: any; // can be anything that will generate uniqueness
    username: string;
    password: string; // hash using bcrypt
  };
  ```

- JWT token should _only_ contain the user ID in it!

2. Add user registration (simple username/password and check for uniqueness & see schema above)

3. Implement URL shortening functionality

- schema

```ts
type Schema = {
  id: any; // can be anything that will generate uniqueness
  original_url: string; // https://example.com
  short_url: string; // helloWorld
  expiry_date?: DateTime; // optional
  created_at: DateTime;
  updated_at: DateTime;
  owner: User; // foreign key relationship
  redirects: number;
};
```

- user can set a custom URL for a shortened link (e.g. `nurl.jwmarb.com/myShortenedUrl`)
- user can add an expiry date to the shortened link
- user can delete the shortened link
- user can view all their shortened links
- user can view the stats of their shortened links (e.g. number of redirects)
- if shortened url already exists, override the previous shortened link and (if it exists in the database) change ownership to user

- Example **PUT** request: `/api/shorten` with the following body:

```json
{
  "url": "https://example.com",
  "customUrl": "helloWorld",
  "expiration": 3600 // 1 hour in seconds
}
```

...with the following headers:

```json
{
  "Authorization": "Bearer myJwtToken"
}
```

The expected behavior is that this creates a shortened link entry in the database with the following fields (note this is JSON, not SQL, for clarity purposes, but the actual database fields would be similar to this example, with appropriate data types and constraints):

```json
{
  "id": "someuniqueid",
  "url": "https://example.com",
  "customUrl": "helloWorld",
  "expiry_date": "2023-04-01T13:00:00Z",
  "owner": "userId", // this should be foreign key relationship to a user in the User table,
  "created_at": "2023-04-01T12:00:00Z",
  "updated_at": "2023-04-01T12:00:00Z",
  "redirects": 0
}
```

- Example **DELETE** request: `/api/shorten` with the following body:

```json
{
  "id": "someuniqueid"
}
```

...with the following headers:

```json
{
  "Authorization": "Bearer myJwtToken"
}
```

The expected behavior is that it deletes the shortened URL entry from the database. Of course you need to check if the authorization token is valid and if the user owns the shortened URL entry before deleting it. There is no JSON response expected for this request and only an HTTP status of OK should be returned.

- Example **GET** request: `/api/shorten`

...with the following headers:

```json
{
  "Authorization": "Bearer myJwtToken"
}
```

The expected behavior is that it returns a list of shortened URLs owned by the user (you can extract the user from the JWT) with the given `userId`. The response should be a JSON array of objects, each containing the `id`, `originalUrl`, and `createdAt` fields. For example:

```json
[
  {
    "id": "someuniqueid",
    "url": "https://example.com",
    "customUrl": "helloWorld",
    "expiry_date": "2023-04-01T13:00:00Z",
    "owner": "userId", // this should be foreign key relationship to a user in the User table,
    "created_at": "2023-04-01T12:00:00Z",
    "updated_at": "2023-04-01T12:00:00Z",
    "redirects": 0
  },
  {
    "id": "anotheruniqueid",
    "url": "https://anotherexample.com",
    "customUrl": "anotherHelloWorld",
    "expiry_date": null,
    "owner": "userId", // this should be foreign key relationship to a user in the User table,
    "created_at": "2023-04-01T12:00:00Z",
    "updated_at": "2023-04-01T12:00:00Z",
    "redirects": 0
  },
  ...
]
```
