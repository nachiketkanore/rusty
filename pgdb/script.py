import psycopg2
from faker import Faker

def insert_data():
    # Replace these values with your actual PostgreSQL connection details
    db_params = {
        "host": "localhost",
        "port": "5432",
        "database": "test",
        "user": "test",
        "password": "test"
    }

    try:
        # Connect to the PostgreSQL database
        connection = psycopg2.connect(**db_params)

        # Create a cursor to perform database operations
        cursor = connection.cursor()

        fake = Faker()

        # Example data to insert into the table
        data_to_insert = [
            (fake.name(), fake.random_int(18, 60)) for _ in range(2)
        ]

        # SQL query to insert data into the table
        insert_query = "INSERT INTO my_table (name, age) VALUES (%s, %s)"

        # Execute the query for each data item
        cursor.executemany(insert_query, data_to_insert)

        # Commit the changes to the database
        connection.commit()

        print("Data inserted successfully!")
        print("Inserted data: \n", data_to_insert)

        # SQL query to fetch all entries from the table
        select_query = "SELECT * FROM my_table"

        # Execute the query
        cursor.execute(select_query)

        # Fetch all rows from the result set
        rows = cursor.fetchall()

        # Print the fetched data
        print("Fetched data:")
        for row in rows:
            print(row)

    except Exception as e:
        print("Error:", e)

    finally:
        # Close the cursor and the database connection
        cursor.close()
        connection.close()

if __name__ == "__main__":
    insert_data()
