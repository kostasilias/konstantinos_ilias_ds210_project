# Konstantinos Ilias  
# DS210 Project Write-Up

## Dataset/Project Description

This project uses data from email communication of employees of Enron, a company that filed for bankruptcy in 2001 because it committed accounting fraud. I found the data here:  
https://snap.stanford.edu/data/email-Enron.html.  
I am using the SNAP Enron dataset(email-Enron (1).txt) which is a dataset of edges where each node is an email address. Although the original email data is directional meaning one user sends a message to another, the SNAP Enron dataset treats the email communication as undirected, meaning an edge exists between two nodes if at least one email was exchanged regardless of its direction.

## Data Processing

The SNAP database provides numeric node IDs with no direct email mapping. However, another dataset was provided (enron_mail_20150507.tar.gz), which contains over 500,000 emails organized in employee folders within a maildir directory, where each folder (e.g., lay-k, skilling-j) represents an Enron employee’s mailbox. Sender and receiver email addresses are provided as well. I used python to create another csv file that maps the numerical nodes to the email addresses and employee folders. With python I produced email_to_node.csv, which mapps NodeID to Email to Folder. Emails and folders can be different as only employees have folders but the dataset has a lot of non Enron emails. Therefore, these non Enron emails are saved in folders of Enron employees.

## What does the code do, how to run it?

The program performs three types of centrality analysis (Degree, Closeness, Betweenness) and finds clusters to identify representative nodes within each cluster. Given that degree centrality is the least computationally heavy to implement, I calculated closeness and betweenness centrality only on the top 1000 nodes with the highest degree centrality. The code prints the top 10 nodes by:  
1) Degree Centrality  
2) Closeness Centrality  
3) Betweenness Centrality  

and also prints cluster leaders (top node by degree in each cluster) and cluster the nodes by all centrality measures using k-means.  
It finally generates the following plots:  
1) degree_histogram.png  
2) closeness_vs_degree.png  
3) betweenness_histogram.png  
4) clusters.png  

I did not create any custom enums or structs as most of the data were simple edges, so I just used standard Rust collections to represent them.

- **Degree Centrality** tells us how many direct connections a node has, indicating how active an individual is in the network.  
- **Closeness Centrality** measures how close a node is to all others via shortest paths, identifying those who can quickly communicate with everyone else.  
- **Betweenness Centrality** captures how often a node lies on the shortest paths between other nodes. These nodes serve as key connectors or information brokers.

To run the code one needs these two datasets:  
- email-Enron (1).txt  
- email_to_node.csv  

These four modules:  
- main.rs: Uses all functions in other modules to calculate different centrality measures and cluster  
- graph.rs: Contains functions that read file, conduct mapping and compute centrality measures  
- cluster.rs: Contains functions that Divide nodes into clusters with the help of BFS and k-means  
- plot.rs: Contains functions that generate plots using the plotters crate  

and an environment that supports Rust and cargo. Using the `cargo run –release > output.txt` command the program takes around 25 seconds to generate the `output.txt` file which contains the output.

## These are some important functions used and what they do:

- `read_file(path: &str) -> Vec<(usize, usize)>`: Reads the edge list from the dataset and returns a list of email communication pairs.
- `load_email_mapping(path: &str) -> HashMap<usize, (String, String)>`: Maps numeric node IDs to actual email addresses and employee folders.
- `compute_degree(edges: &[(usize, usize)]) -> HashMap<usize, usize>`: Calculates the degree (number of direct connections) for each node.
- `compute_closeness(edges: &[(usize, usize)], nodes: &HashSet<usize>) -> HashMap<usize, f64>`: Computes closeness centrality by evaluating shortest path distances.
- `compute_betweenness(edges: &[(usize, usize)], nodes: &HashSet<usize>) -> HashMap<usize, f64>`: Calculates betweenness centrality by counting shortest paths passing through each node.
- `find_clusters(edges: &[(usize, usize)]) -> Vec<HashSet<usize>>`: Identifies clusters of connected nodes using breadth-first search (BFS).
- `kmeans(features: &HashMap<usize, (f64, f64, f64)>, k: usize, max_iters: usize) -> HashMap<usize, usize>`: Performs K-Means clustering on degree, closeness, and betweenness centrality of each node to assign nodes to clusters.
- `normalize_features(features: &mut HashMap<usize, (f64, f64, f64)>)`: Normalizes features to ensure equal weighting during clustering.

## Tests

Inside the `graph` and `cluster` modules there is a module called `tests` that contains tests for the functions of each module. All these tests ensure that the quantitative methods used to analyze the Enron email nodes work as expected.

- `test_compute_degree`: Verifies degree centrality calculation is accurate by using a small graph.
- `test_compute_closeness`: Ensures nodes more central in the graph have higher closeness by using a small graph.
- `test_compute_betweenness`: Validates that betweenness is higher for bridge nodes than edge notes in a small graph.
- `test_find_clusters`: Checks that clusters are correctly separated in a small graph.
- `test_normalize_features`: Confirms feature normalization works as intended.
- `test_kmeans`: Verifies that kmeans has divided a small set of points into the correct clusters.

## What is the output?

The code produces rankings of the most important individuals in the Enron email network based on three centrality metrics. Nodes with the highest degree centrality (e.g., bdbinford@aol.com, .marisha@enron.com) had the most direct email connections, meaning they were highly active in sending and receiving emails. Nodes with high closeness centrality could reach others quickly in the network. Those with high betweenness centrality often served as connections between different groups, playing a role in information flow and inter-group communication. Additionally, the network was divided into clusters of connected individuals, and the most connected node in each cluster was identified as its leader, revealing the most central figure within each communication cluster. Nodes were also divided into clusters based on their centrality measures using k-means.

These four executives got convicted of fraud: Jeffrey Skilling – Former CEO, Andrew Fastow – Former CFO, Kenneth Lay – Former Chairman and CEO and Richard Causey – Former Chief Accounting Officer. Based on the output of the code: Jeffrey Skilling appears in Closeness Centrality and Cluster 4 based on degree and Kenneth Lay appears in Betweenness Centrality. On the clusters generated with k-means, emails that belong to their folders appear in multiple clusters. However, most emails on Skilling’s folder appear to be concentrated in Cluster 2.

