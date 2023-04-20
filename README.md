# rust-question-api

My endeavour to learn Rust started a couple of weeks ago and it's been quite interesting.

This project is the first one I created following the book "Rust web development" [https://www.manning.com/books/rust-web-development]. Interesting book that let you build from start to finish a small api, BUT, at the end the project does not fully work as expected.

Here, you can see some differences from the original project that I needed to add to make it work as expected. Hope that it can help someone else!

## Test the endpoints

You can use the following curl scripts to test. Don't forget to rename the "<auth-token>" and pay atention on the ids!

### get questions
```
curl --location 'localhost:8080/questions'
```

### post a question
```
curl --location 'localhost:8080/questions' \
--header 'Content-Type: application/json' \
--header 'Authorization: <auth-token>' \
--data '{
    "id": "2",
    "title": "New question",
    "content": "brexit was a shitty idea!",
    "tags": [
        "general"
    ]
}'
```

### update question
```
curl --location --request PUT 'localhost:8080/questions/1' \
--header 'Content-Type: application/json' \
--header 'Authorization: <auth-token>' \
--data '{
    "id": 1,
    "title": "bleble",
    "content": "OLD CONTENT",
    "tags": [
        "general"
    ]
}'
```

### delete question
```
curl --location --request DELETE 'localhost:8080/questions/1' \
--header 'Authorization: <auth-token>'
```

### post answer
```
curl --location 'localhost:8080/answers' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--header 'Authorization: <auth-token>' \
--data-urlencode 'content=blabla' \
--data-urlencode 'question_id=1'
```

### user registration
```
curl --location 'localhost:8080/registration' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "test@email.com",
    "password": "somepass"
}'
```

### user login
```
curl --location 'localhost:8080/login' \
--header 'Content-Type: application/json' \
--data-raw '{
      "email": "test@email.com",
      "password": "somepass"
  }'
```
