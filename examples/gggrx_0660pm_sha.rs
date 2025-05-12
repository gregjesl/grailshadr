use grailshadr::GGGRX_0660PM_SHA;

fn main()
{
    println!("Checking available SHADRS...");
    assert!(grailshadr::AVAILABLE_SHADRS.contains(&GGGRX_0660PM_SHA));
    println!("gggrx_0660pm_sha found!");
    println!("Printing first 6 lines...");
    for line in GGGRX_0660PM_SHA.lines().take(6)
    {
        println!("{line}");
    }
}