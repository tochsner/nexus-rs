use nexus::parse_file;

fn main() {
    for _ in 1..2 {
        parse_file(
            // "/Users/tobiaochsner/Downloads/trees/bouckaert_et_al2018/original/pny10.fixed.cov.ucln.bdsky.ba-sp.trees",
            "/Users/tobiaochsner/Documents/Thesis/Validation/data/mcmc_runs/yule-50_98.trees",
        );
    }
}
