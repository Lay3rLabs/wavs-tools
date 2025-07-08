Start up:

    `task backend:start CHAINS=N OPERATORS=M`

Intended workflow:
	- dev: deploy original service manager
	- dev: create empty service json (no workflows)
	- register 1 operator to it (post /app + register)
	- dev: deploy mirror contract on chain 2 (with operators)
	- dev: deploy any service handlers
	- dev: build full service json and set service uri
	- register remaining operators
	- query that mirror is synced