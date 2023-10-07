use crate::{
    host,
    mail::{self, Transmission},
};
use anyhow::Result;
use csv;
use dashmap::DashMap;
use indicatif::ProgressBar;
use log::{error, info};
use rayon::prelude::*;
use std::path::Path;

pub(crate) fn list(output: &str, input: &str) -> Result<()> {
    let files = host::list_files(input, 1).unwrap();

    let from_map: DashMap<mail::Address, u32> = DashMap::new();
    let to_map: DashMap<mail::Address, u32> = DashMap::new();
    let cc_map: DashMap<mail::Address, u32> = DashMap::new();
    let bcc_map: DashMap<mail::Address, u32> = DashMap::new();

    let bar = ProgressBar::new(files.len().try_into().unwrap());

    files.par_iter().for_each(|f| {
        if let Ok(m) = mail::read_eml_headers(f) {
            for from_addr in m.from {
                *from_map.entry(from_addr).or_default() += 1;
            }

            for to_addr in m.to {
                *to_map.entry(to_addr).or_default() += 1;
            }

            for cc_addr in m.cc {
                *cc_map.entry(cc_addr).or_default() += 1;
            }

            for bcc_addr in m.bcc {
                *bcc_map.entry(bcc_addr).or_default() += 1;
            }

            bar.inc(1);
        }
    });

    bar.finish();

    info!(
        r#"Extracted entries:
 from: {}
   to: {}
   cc: {}
  bcc: {}
"#,
        from_map.len(),
        to_map.len(),
        cc_map.len(),
        bcc_map.len()
    );

    let mut wtr = csv::WriterBuilder::new().from_path(output)?;
    wtr.write_record(&["address", "name", "count", "kind"])?;

    for (address, count) in from_map {
        wtr.write_record(&[
            address.addr,
            address.name.unwrap_or_else(|| "".to_string()),
            count.to_string(),
            "from".to_string(),
        ])?;
    }

    for (address, count) in to_map {
        wtr.write_record(&[
            address.addr,
            address.name.unwrap_or_else(|| "".to_string()),
            count.to_string(),
            "to".to_string(),
        ])?;
    }

    for (address, count) in cc_map {
        wtr.write_record(&[
            address.addr,
            address.name.unwrap_or_else(|| "".to_string()),
            count.to_string(),
            "cc".to_string(),
        ])?;
    }

    for (address, count) in bcc_map {
        wtr.write_record(&[
            address.addr,
            address.name.unwrap_or_else(|| "".to_string()),
            count.to_string(),
            "bcc".to_string(),
        ])?;
    }

    wtr.flush()?;

    Ok(())
}

pub(crate) fn network(output: &str, input: &str) -> Result<()> {
    let files = host::list_files(input, 1).unwrap();

    let bar = ProgressBar::new(files.len().try_into().unwrap());

    let results = files
        .par_iter()
        .map(|f| {
            if let Ok(m) = mail::eml_transmissions(f) {
                bar.inc(1);
                m
            } else {
                bar.inc(1);
                vec![]
            }
        })
        .flatten()
        .collect::<Vec<Transmission>>();

    bar.finish();

    info!("Extracted {} transmissions", results.len());

    let mut wtr = csv::WriterBuilder::new().from_path(output)?;
    wtr.write_record(&["source", "target", "date", "kind"])?;

    for t in results {
        wtr.write_record(&[t.source.addr, t.target.addr, t.date, t.kind.to_string()])?;
    }

    wtr.flush()?;

    Ok(())
}

pub(crate) fn attachment(outdir: &str, input: &str) -> Result<()> {
    let files = host::list_files(input, 1).unwrap();
    let outdir = Path::new(outdir).to_path_buf();

    let bar = ProgressBar::new(files.len().try_into().unwrap());

    let results = files
        .par_iter()
        .map(|f| match mail::extract_attachments(f, &outdir) {
            Err(_) => {
                bar.inc(1);
                error!("{} failed to extract attachments", f.to_string_lossy());
                vec![f.to_string_lossy().to_string()]
            }
            _ => {
                bar.inc(1);
                vec![]
            }
        })
        .flatten()
        .collect::<Vec<String>>();

    bar.finish();

    results.iter().for_each(|r| println!("{}", r));

    Ok(())
}
