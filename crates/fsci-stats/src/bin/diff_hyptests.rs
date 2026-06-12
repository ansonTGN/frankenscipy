use fsci_stats::*;
fn t(name: &str, s: f64, p: f64) {
    println!("{name},{s:.17e},{p:.17e}");
}
fn main() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let y = [2.0, 3.0, 3.0, 5.0, 6.0, 8.0, 9.0, 10.0];
    let z = [5.0, 3.0, 8.0, 1.0, 9.0, 2.0, 7.0, 4.0];
    // correlation
    let r = pearsonr(&x, &y);
    t("pearsonr", r.statistic, r.pvalue);
    let r = spearmanr(&x, &z);
    t("spearmanr", r.statistic, r.pvalue);
    let r = kendalltau(&x, &z);
    t("kendalltau", r.statistic, r.pvalue);
    // two-sample
    let r = mannwhitneyu(&x, &y);
    t("mannwhitneyu", r.statistic, r.pvalue);
    let r = ranksums(&x, &y);
    t("ranksums", r.statistic, r.pvalue);
    let r = wilcoxon(&x, &y);
    t("wilcoxon", r.statistic, r.pvalue);
    let r = brunnermunzel(&x, &y);
    t("brunnermunzel", r.statistic, r.pvalue);
    let r = mood(&x, &y);
    t("mood", r.statistic, r.pvalue);
    let r = ansari(&x, &y);
    t("ansari", r.statistic, r.pvalue);
    // variance / group
    let r = levene(&[&x, &y, &z]);
    t("levene", r.statistic, r.pvalue);
    let r = bartlett(&[&x, &y, &z]);
    t("bartlett", r.statistic, r.pvalue);
    let r = fligner(&[&x, &y, &z]);
    t("fligner", r.statistic, r.pvalue);
    let r = f_oneway(&[&x, &y, &z]);
    t("f_oneway", r.statistic, r.pvalue);
    let r = kruskal(&[&x, &y, &z]);
    t("kruskal", r.statistic, r.pvalue);
    // tie-heavy inputs
    let xt = [1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0];
    let yt = [1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0];
    let r = mannwhitneyu(&xt, &yt);
    t("mannwhitneyu_ties", r.statistic, r.pvalue);
    let r = spearmanr(&xt, &yt);
    t("spearmanr_ties", r.statistic, r.pvalue);
    let r = kendalltau(&xt, &yt);
    t("kendalltau_ties", r.statistic, r.pvalue);
    let r = ks_2samp(&x, &z);
    t("ks_2samp", r.statistic, r.pvalue);
}
