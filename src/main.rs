#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate inflate_0_3_4;
extern crate snap_0_2_4;


criterion_group!(inflate_0_3_4, inflate_0_3_4::decode);
criterion_group!(snap_0_2_4_rust,
    snap_0_2_4::rust::zflat00_html,
    snap_0_2_4::rust::zflat01_urls,
    snap_0_2_4::rust::zflat02_jpg,
    snap_0_2_4::rust::zflat03_jpg_200,
    snap_0_2_4::rust::zflat04_pdf,
    snap_0_2_4::rust::zflat05_html4,
    snap_0_2_4::rust::zflat06_txt1,
    snap_0_2_4::rust::zflat07_txt2,
    snap_0_2_4::rust::zflat08_txt3,
    snap_0_2_4::rust::zflat09_txt4,
    snap_0_2_4::rust::zflat10_pb,
    snap_0_2_4::rust::zflat11_gaviota,

    snap_0_2_4::rust::uflat00_html,
    snap_0_2_4::rust::uflat01_urls,
    snap_0_2_4::rust::uflat02_jpg,
    snap_0_2_4::rust::uflat03_jpg_200,
    snap_0_2_4::rust::uflat04_pdf,
    snap_0_2_4::rust::uflat05_html4,
    snap_0_2_4::rust::uflat06_txt1,
    snap_0_2_4::rust::uflat07_txt2,
    snap_0_2_4::rust::uflat08_txt3,
    snap_0_2_4::rust::uflat09_txt4,
    snap_0_2_4::rust::uflat10_pb,
    snap_0_2_4::rust::uflat11_gaviota);

criterion_main!(inflate_0_3_4, snap_0_2_4_rust);
