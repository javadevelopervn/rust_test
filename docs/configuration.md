# Configuration

The Rust Load Tester is configured using a YAML file. This file defines a series of steps that are executed sequentially.

## Top-level configuration

The top-level configuration object has a single key: `steps`.

- `steps`: A list of test steps to be executed.

```yaml
steps:
  - ...
  - ...
```

## Test Step

Each test step is an object with the following keys:

- `name`: The name of the test step.
- `request`: The request to be made.
- `concurrency`: The number of concurrent users. Defaults to 1.
- `iterations`: The number of iterations for each concurrent user. Defaults to 1.

```yaml
- name: "Get user"
  request:
    ...
  concurrency: 10
  iterations: 100
```

## Request

The `request` object defines the HTTP request to be made. It has the following keys:

- `method`: The HTTP method (e.g., GET, POST).
- `url`: The URL of the request.
- `headers`: A list of headers to be sent with the request.
- `body`: The body of the request.

```yaml
request:
  method: POST
  url: "https://jsonplaceholder.typicode.com/posts"
  headers:
    - ["content-type", "application/json; charset=UTF-8"]
  body: |
    {
      "title": "foo",
      "body": "bar",
      "userId": 1
    }
```

## Templating

The tool supports a very simple form of templating. You can use `{{response}}` in the body of a request to insert the response from the previous step.

```yaml
body: |
  {
    "title": "foo",
    "body": "bar",
    "userId": 1,
    "previous_response": "{{response}}"
  }
```

This allows you to chain requests together, using the output of one request as the input for the next.
