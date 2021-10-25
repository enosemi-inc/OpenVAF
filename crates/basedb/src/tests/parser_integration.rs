use crate::{tests::TestDataBase, BaseDB};
use sourcegen::{project_root, skip_slow_tests};
use std::{fs::read_to_string, path::PathBuf};
#[test]
fn bsim6() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIM6/bsim6.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIM6").join("bsim6.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsim6.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsimbulk() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIMBULK/bsimbulk.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIMBULK").join("bsimbulk.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimbulk.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsimcmg() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIMCMG/bsimcmg.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIMCMG").join("bsimcmg.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg.va", &file_contents);
    }
    {
        let path = project_root()
            .join("integration_tests")
            .join("BSIMCMG")
            .join("bsimcmg_binning_parameters.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg_binning_parameters.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMCMG").join("bsimcmg_body.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg_body.include", &file_contents);
    }
    {
        let path = project_root()
            .join("integration_tests")
            .join("BSIMCMG")
            .join("bsimcmg_cfringe.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg_cfringe.include", &file_contents);
    }
    {
        let path = project_root()
            .join("integration_tests")
            .join("BSIMCMG")
            .join("bsimcmg_quasi_static_cv.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg_quasi_static_cv.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMCMG").join("bsimcmg_rdsmod.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimcmg_rdsmod.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMCMG").join("common_defs.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/common_defs.include", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsimimg() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIMIMG/bsimimg.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIMIMG").join("bsimimg.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimimg.va", &file_contents);
    }
    {
        let path = project_root()
            .join("integration_tests")
            .join("BSIMIMG")
            .join("bsimimg_binning.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimimg_binning.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMIMG").join("bsimimg_body.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimimg_body.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMIMG").join("bsimimg_sp.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimimg_sp.include", &file_contents);
    }
    {
        let path =
            project_root().join("integration_tests").join("BSIMIMG").join("common_defs.include");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/common_defs.include", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsimsoi() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIMSOI/bsimsoi.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIMSOI").join("bsimsoi.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsimsoi.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn diode() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/DIODE/diode.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("DIODE").join("diode.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/diode.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn hicuml2() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/HICUML2/hicuml2.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("HICUML2").join("hicuml2.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/hicuml2.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsim3() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIM3/bsim3.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIM3").join("bsim3.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsim3.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn bsim4() {
    if skip_slow_tests() {
        return;
    }
    let root_file = read_to_string(PathBuf::from(
        "/home/dspom/Projects/OpenVAF/integration_tests/BSIM4/bsim4.va",
    ))
    .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("BSIM4").join("bsim4.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/bsim4.va", &file_contents);
    }
    db.parse_and_check();
}
#[test]
fn ekv() {
    if skip_slow_tests() {
        return;
    }
    let root_file =
        read_to_string(PathBuf::from("/home/dspom/Projects/OpenVAF/integration_tests/EKV/ekv.va"))
            .unwrap();
    let db = TestDataBase::new("/root.va", &root_file);
    {
        let path = project_root().join("integration_tests").join("EKV").join("ekv.va");
        let file_contents = read_to_string(path).unwrap();
        db.vfs().borrow_mut().add_virt_file("/ekv.va", &file_contents);
    }
    db.parse_and_check();
}
