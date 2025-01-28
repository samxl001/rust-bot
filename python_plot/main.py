import sqlite3
import matplotlib.pyplot as plt
import os
import shutil

def fetch_data_from_db(database_filename):
    # Connect to the SQLite database
    conn = sqlite3.connect(database_filename)
    
    # Create a cursor object to interact with the database
    cursor = conn.cursor()
    
    # Query to get the column names (field names) of the "mentions" table
    cursor.execute("PRAGMA table_info(mentions);")
    columns = cursor.fetchall()
    
    # Extract field names, excluding 'date'
    field_names = [column[1] for column in columns if column[1] != 'date']
    
    # Query to get all the data from the "mentions" table
    cursor.execute(f"SELECT date, {', '.join(field_names)} FROM mentions;")
    rows = cursor.fetchall()
    
    # Prepare the data to be returned
    x_data = []  # Dates
    y_data = {field: [] for field in field_names}  # Field values for each field
    
    for row in rows:
        x_data.append(row[0])  # The date (first column)
        for i, field in enumerate(field_names):
            y_data[field].append(row[i + 1])  # The field values (subsequent columns)
    
    # Close the connection
    conn.close()
    
    # Return the data
    return x_data, y_data


def plot_line_graph(x_data, y_data, output_file):
    # Convert date strings to numbers (if required), or simply use the index as x
    x_indices = list(range(len(x_data)))
    
    # Plot the data for each field
    plt.figure(figsize=(10, 6))
    for field, values in y_data.items():
        plt.plot(x_indices, values, label=field)  # Plotting each field's data
    
    # Customize the plot
    plt.title("Unique Mentions by Name over Time on Reddit (r/all/hot)")
    plt.xlabel("Date")
    plt.ylabel("Mentions")
    
    # Set x-ticks to correspond to the date values
    plt.xticks(x_indices, x_data, rotation=45)  # Rotate x labels for better readability
    plt.tight_layout()  # Adjust layout to avoid label overlap
    
    # Show the legend
    plt.legend()

    # Save the plot as a PNG file
    plt.savefig(output_file)
    print(f"Graph saved as {output_file}")
    
def copy_file(database_filename):
    cur_dir = os.getcwd()
    parent_dir = os.path.dirname(cur_dir)
    source = parent_dir + '/' + database_filename
    destination = cur_dir + '/' + database_filename
    shutil.copy(source, destination)
    
    

def main():
    database_filename = 'data.db3'  # Replace with your database file
    #copy_file(database_filename)
    x_data, y_data = fetch_data_from_db(database_filename)
    # Plot the graph and save it as PNG
    plot_line_graph(x_data, y_data, 'result.png')
    #os.remove(database_filename)

main()