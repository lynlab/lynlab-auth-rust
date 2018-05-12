# API Specification
## List

  - POST /v1/register


## Error Response Model

```json
{
  "error": "string"
}
```

  - `error` : Error code.


## POST /v1/register

Register new account.

### Request Body

```json
{
  "username": "string",
  "password": "string",
  "email": "string"
}
```

### Response
  - **200 OK** : The account added successfully and the confirmation email sent.
  - **400 BAD REQUEST** : Failed to register.
    - `AU0001` : Duplicated username or email.
