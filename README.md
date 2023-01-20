# OJP Meta Service

## Development

Make sure you have at least rust `1.64.0` (stable) installed.

Copy the dotenv template:

```sh
cp .env.example .env
```

replace actual tokens -> only the swiss system uses tokens so far.

to run the application: `cargo run`

to run tests: `cargo test`

## Testing endpoints

If you want to test endpoints you can load `docs/ojp_insomnia.json` into your Insomnia and use those requests there.

If you want to test only our endpoints you can run the system with `cargo run` and then run requests agains `http://localhost:8000`.

Our most advanced endpoint is the `trip_exchange` POST endpoint. I takes request data in this form:

```json
{
	"origin": {
		"name": "Bern",
		"reference": "8507000",
		"system": "ch"
	},
	"destination": {
		"name": "Wien Hauptbahnhof",
		"reference": "U3xBPTFATz1XaWVuIEhhdXB0YmFobmhvZkBYPTE2Mzc2NDEzQFk9NDgxODUxODRAVT04MUBMPTQ5MDEzNDkwMEBCPTFAcD0xNjY1MTMyOTEyQGk9QcOXYXQ6NDk6MTM0OUB8V2llbiBIYXVwdGJhaG5ob2Z8MTYuMzc2NDEzfDQ4LjE4NTE4NHx0cnVl",
		"system": "at"
	},
	"exchange": "ch:1:sloid:3000"
}
```

and should return a tuple in the form of `(trips from origin to exchange point, trips from exchange point to destination)`