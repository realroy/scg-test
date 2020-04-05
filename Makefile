start:
	docker-compose up --build -d
	open http://localhost:3000
stop:
	docker-compose down
