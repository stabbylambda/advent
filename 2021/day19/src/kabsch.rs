use nalgebra::{DMatrix, Matrix3, Vector3};

pub(crate) fn kabsch(p: DMatrix<f64>, q: DMatrix<f64>) -> nalgebra::DMatrix<f64> {
    let pm = p.mean();
    let qm = q.mean();

    let pc = DMatrix::from_element(p.nrows(), p.ncols(), pm);
    let qc = DMatrix::from_element(q.nrows(), q.ncols(), qm);

    let p1 = &p - pc;
    let q1 = &q - qc;

    let h = p1.transpose() * q1;
    let svd = h.svd(true, true);

    let v = svd.v_t.unwrap().transpose();
    let u_t = svd.u.unwrap().transpose();

    let d = if (&v * &u_t).determinant() > 0.0 {
        1.0
    } else {
        -1.0
    };

    let e = Matrix3::from_diagonal(&Vector3::new(1.0, 1.0, d));

    let r = v * e * u_t;
    p * r
}
