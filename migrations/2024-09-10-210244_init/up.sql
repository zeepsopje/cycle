CREATE TABLE projects (
	id INT PRIMARY KEY AUTO_INCREMENT,
	name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE timers (
	id INT PRIMARY KEY AUTO_INCREMENT,
	project_id INT NOT NULL,
	name VARCHAR(255) NOT NULL UNIQUE,

	FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE time_entries (
	id INT PRIMARY KEY AUTO_INCREMENT,
	timer_id INT NOT NULL,
	start_time DATETIME NOT NULL,
	end_time DATETIME NOT NULL,

	FOREIGN KEY (timer_id) REFERENCES timers(id) ON DELETE CASCADE
);
