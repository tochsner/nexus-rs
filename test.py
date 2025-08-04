import nexus
import commonnexus
import time

start = time.time()

for _ in range(10):
    nexus.parse_file("/Users/tobiaochsner/Documents/Thesis/Validation/data/mcmc_runs/yule-50_98.trees")

end = time.time()

print("Time for nexus: ", end - start)

start = time.time()

for _ in range(10):
    nex = commonnexus.Nexus.from_file("/Users/tobiaochsner/Documents/Thesis/Validation/data/mcmc_runs/yule-50_98.trees")

end = time.time()

print("Time for commonnexus: ", end - start)
