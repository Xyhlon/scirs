//! Linear algebra functions
//!
//! This module provides functions for linear algebra operations,
//! including BLAS and LAPACK wrappers, decompositions, and other utilities.
//!
//! ## Overview
//!
//! * Basic matrix operations - determinants, inverses, etc.
//! * Matrix decomposition - LU, QR, SVD, Cholesky
//! * Eigenvalue/eigenvector computations
//! * Matrix functions - matrix exponential, square root, etc.
//! * Matrix norms and condition numbers
//! * Linear equation solvers - direct and iterative methods
//! * Special matrix functions
//! * Accelerated implementations using native BLAS/LAPACK libraries
//!
//! ## Examples
//!
//! Basic operations:
//!
//! ```
//! use ndarray::array;
//! use scirs2_linalg::{det, inv};
//!
//! // Compute determinant
//! let a = array![[1.0_f64, 2.0], [3.0, 4.0]];
//! let d = det(&a.view()).unwrap();
//! assert!((d - (-2.0)).abs() < 1e-10);
//!
//! // Compute inverse
//! let a_inv = inv(&a.view()).unwrap();
//! ```
//!
//! Matrix decompositions:
//!
//! ```
//! use ndarray::array;
//! use scirs2_linalg::{lu, qr, svd, cholesky};
//!
//! let a = array![[1.0_f64, 2.0], [3.0, 4.0]];
//!
//! // LU decomposition
//! let (p, l, u) = lu(&a.view()).unwrap();
//!
//! // QR decomposition
//! let (q, r) = qr(&a.view()).unwrap();
//! ```
//!
//! Iterative solvers:
//!
//! ```
//! use ndarray::array;
//! use scirs2_linalg::conjugate_gradient;
//!
//! let a = array![[4.0_f64, 1.0], [1.0, 3.0]]; // Symmetric positive definite
//! let b = array![1.0_f64, 2.0];
//!
//! // Solve using conjugate gradient
//! let x = conjugate_gradient(&a.view(), &b.view(), 10, 1e-10).unwrap();
//! ```
//!
//! Matrix functions:
//!
//! ```
//! use ndarray::array;
//! use scirs2_linalg::matrix_functions::expm;
//!
//! // Compute matrix exponential
//! let a = array![[0.0_f64, 1.0], [-1.0, 0.0]]; // Rotation matrix
//! let exp_a = expm(&a.view()).unwrap();
//! ```
//!
//! Accelerated operations using native BLAS/LAPACK:
//!
//! ```
//! use ndarray::array;
//! use scirs2_linalg::blas_accelerated;
//!
//! let a = array![[1.0_f64, 2.0], [3.0, 4.0]];
//! let b = array![[5.0_f64, 6.0], [7.0, 8.0]];
//!
//! // Fast matrix multiplication for large matrices
//! let c = blas_accelerated::matmul(&a.view(), &b.view()).unwrap();
//! ```
extern crate blas as _;
extern crate openblas_src;

// Export error types
pub mod error;
pub use error::{LinalgError, LinalgResult};

// Basic modules
pub mod attention;
mod basic;
pub mod batch;
pub mod broadcast;
pub mod complex;
pub mod convolution;
mod decomposition;
// Main eigen module
pub mod eigen;
pub use self::eigen::{eig, eigh, eigvals, eigvalsh, power_iteration};

// Specialized eigen solvers in separate module
pub mod eigen_specialized {
    pub mod banded;
    pub mod sparse;
    pub mod symmetric;
    pub mod tridiagonal;
    pub use banded::{banded_eigh, banded_eigvalsh};
    pub use sparse::{largest_k_eigh, smallest_k_eigh};
    pub use symmetric::{symmetric_eigh, symmetric_eigvalsh};
    pub use tridiagonal::{tridiagonal_eigh, tridiagonal_eigvalsh};
}
pub mod extended_precision;
pub mod generic;
pub mod gradient;
mod iterative_solvers;
pub mod kronecker;
pub mod lowrank;
pub mod matrix_calculus;
pub mod matrix_factorization;
pub mod matrix_functions;
pub mod matrixfree;
pub mod mixed_precision;
mod norm;
pub mod optim;
pub mod perf_opt;
pub mod projection;
/// Quantization-aware linear algebra operations
pub mod quantization;
pub use self::quantization::calibration::{
    calibrate_matrix, calibrate_vector, get_activation_calibration_config,
    get_weight_calibration_config, CalibrationConfig, CalibrationMethod,
};
pub mod random;
pub mod random_matrices;
// 一時的にrandom_newモジュールを無効化（コンパイル問題解決まで）
// pub mod random_new;
mod diagnostics;
pub mod simd_ops;
mod solve;
pub mod sparse_dense;
pub mod special;
pub mod specialized;
pub mod stats;
pub mod structured;
#[cfg(feature = "tensor_contraction")]
pub mod tensor_contraction;
// Automatic differentiation support
#[cfg(feature = "autograd")]
pub mod autograd;

// SciPy-compatible API wrappers
pub mod compat;

// Accelerated implementations using BLAS/LAPACK
pub mod blas_accelerated;
pub mod lapack_accelerated;

// BLAS and LAPACK wrappers
pub mod blas;
pub mod lapack;

// Re-export the accelerated implementations
pub mod accelerated {
    //! Accelerated linear algebra operations using native BLAS/LAPACK
    //!
    //! This module provides optimized implementations of linear algebra operations
    //! using ndarray-linalg's bindings to native BLAS/LAPACK libraries.
    //! These functions are significantly faster for large matrices compared to
    //! pure Rust implementations.

    pub use super::blas_accelerated::*;
    pub use super::lapack_accelerated::*;
}
// accelerated実装は既にpubモジュールとして公開されているので、ここでの再エクスポートは不要

// Re-exports for user convenience
pub use self::basic::{det, inv, matrix_power};
pub use self::eigen_specialized::sparse::{largest_k_eigh, smallest_k_eigh};
// Re-export complex module functions explicitly to avoid conflicts
pub use self::complex::enhanced_ops::{
    det as complex_det, frobenius_norm, hermitian_part, inner_product, is_hermitian, is_unitary,
    matrix_exp, matvec, polar_decomposition, power_method, rank as complex_rank,
    schur as complex_schur, skew_hermitian_part, trace,
};
pub use self::complex::{complex_inverse, complex_matmul, hermitian_transpose};
pub use self::decomposition::*;
// Eigen module exports included in other use statements
pub use self::eigen_specialized::*;
pub use self::extended_precision::*;
pub use self::iterative_solvers::*;
pub use self::matrix_calculus::*;
pub use self::matrix_factorization::{
    cur_decomposition, interpolative_decomposition, nmf, rank_revealing_qr, utv_decomposition,
};
pub use self::matrixfree::{
    block_diagonal_operator, conjugate_gradient as matrix_free_conjugate_gradient,
    diagonal_operator, gmres as matrix_free_gmres, jacobi_preconditioner,
    preconditioned_conjugate_gradient as matrix_free_preconditioned_conjugate_gradient,
    LinearOperator, MatrixFreeOp,
};
pub use self::norm::*;
pub use self::solve::*;
pub use self::specialized::{
    specialized_to_operator, BandedMatrix, SpecializedMatrix, SymmetricMatrix, TridiagonalMatrix,
};
pub use self::stats::*;
pub use self::structured::{
    structured_to_operator, CirculantMatrix, HankelMatrix, StructuredMatrix, ToeplitzMatrix,
};
#[cfg(feature = "tensor_contraction")]
pub use self::tensor_contraction::{batch_matmul, contract, einsum, hosvd};

// Prelude module for convenient imports
pub mod prelude {
    //! Common linear algebra operations for convenient importing
    //!
    //! ```
    //! use scirs2_linalg::prelude::*;
    //! ```

    // Pure Rust implementations
    pub use super::attention::{
        attention, attention_with_alibi, attention_with_rpe, causal_attention, flash_attention,
        grouped_query_attention, linear_attention, masked_attention, multi_head_attention,
        relative_position_attention, rotary_embedding, scaled_dot_product_attention,
        sparse_attention, AttentionConfig, AttentionMask,
    };
    pub use super::basic::{det, inv};
    pub use super::batch::attention::{
        batch_flash_attention, batch_multi_head_attention, batch_multi_query_attention,
    };
    pub use super::broadcast::{
        broadcast_matmul, broadcast_matmul_3d, broadcast_matvec, BroadcastExt,
    };
    pub use super::complex::enhanced_ops::{
        det as complex_det, frobenius_norm as complex_frobenius_norm, hermitian_part,
        inner_product as complex_inner_product, is_hermitian, is_unitary,
        matrix_exp as complex_exp, matvec as complex_matvec, polar_decomposition as complex_polar,
        schur as complex_schur, skew_hermitian_part,
    };
    pub use super::complex::{
        complex_inverse, complex_matmul, complex_norm_frobenius, hermitian_transpose,
    };
    pub use super::convolution::{
        col2im, compute_conv_indices, conv2d_backward_bias, conv2d_backward_input,
        conv2d_backward_kernel, conv2d_im2col, conv_transpose2d, im2col, max_pool2d,
        max_pool2d_backward,
    };
    pub use super::decomposition::{cholesky, lu, qr, svd};
    pub use super::eigen::{eig, eigh, eigvals, eigvalsh, power_iteration};
    pub use super::eigen_specialized::banded::{banded_eigh, banded_eigvalsh};
    pub use super::eigen_specialized::sparse::{largest_k_eigh, smallest_k_eigh};
    pub use super::eigen_specialized::symmetric::{symmetric_eigh, symmetric_eigvalsh};
    pub use super::eigen_specialized::tridiagonal::{tridiagonal_eigh, tridiagonal_eigvalsh};
    pub use super::extended_precision::eigen::{
        extended_eig, extended_eigh, extended_eigvals, extended_eigvalsh,
    };
    pub use super::extended_precision::factorizations::{
        extended_cholesky, extended_lu, extended_qr, extended_svd,
    };
    pub use super::extended_precision::{
        extended_det, extended_matmul, extended_matvec, extended_solve,
    };
    pub use super::iterative_solvers::{
        bicgstab, conjugate_gradient, gauss_seidel, geometric_multigrid, jacobi_method, minres,
        successive_over_relaxation,
    };
    pub use super::kronecker::{
        kfac_factorization, kfac_update, kron, kron_factorize, kron_matmul, kron_matvec,
    };
    pub use super::lowrank::{nmf as lowrank_nmf, pca, randomized_svd, truncated_svd};
    pub use super::matrix_calculus::enhanced::{
        hessian_vector_product, jacobian_vector_product, matrix_gradient, taylor_approximation,
        vector_jacobian_product,
    };
    pub use super::matrix_calculus::{directional_derivative, gradient, hessian, jacobian};
    pub use super::matrix_factorization::{
        cur_decomposition, interpolative_decomposition, nmf, rank_revealing_qr, utv_decomposition,
    };
    pub use super::matrix_functions::{expm, logm, matrix_power, sqrtm};
    pub use super::matrixfree::{
        block_diagonal_operator, conjugate_gradient as matrix_free_conjugate_gradient,
        diagonal_operator, gmres as matrix_free_gmres, jacobi_preconditioner,
        preconditioned_conjugate_gradient as matrix_free_preconditioned_conjugate_gradient,
        LinearOperator, MatrixFreeOp,
    };
    pub use super::mixed_precision::{
        convert, convert_2d, iterative_refinement_solve, mixed_precision_cholesky,
        mixed_precision_cond, mixed_precision_det, mixed_precision_dot, mixed_precision_inv,
        mixed_precision_lstsq, mixed_precision_lu, mixed_precision_matmul, mixed_precision_matvec,
        mixed_precision_qr, mixed_precision_solve, mixed_precision_svd, MixedPrecisionLstsqResult,
    };
    #[cfg(feature = "simd")]
    pub use super::mixed_precision::{
        simd_mixed_precision_dot_f32_f64, simd_mixed_precision_matmul_f32_f64,
        simd_mixed_precision_matvec_f32_f64,
    };
    pub use super::norm::{cond, matrix_norm, matrix_rank, vector_norm};
    pub use super::optim::{block_matmul, strassen_matmul, tiled_matmul};
    pub use super::perf_opt::{
        blocked_matmul, inplace_add, inplace_scale, matmul_benchmark, optimized_transpose,
        OptAlgorithm, OptConfig,
    };
    pub use super::projection::{
        gaussian_random_matrix, johnson_lindenstrauss_min_dim, johnson_lindenstrauss_transform,
        project, sparse_random_matrix, very_sparse_random_matrix,
    };
    pub use super::quantization::calibration::{
        calibrate_matrix, calibrate_vector, CalibrationConfig, CalibrationMethod,
    };
    #[cfg(feature = "simd")]
    pub use super::quantization::simd::{
        simd_quantized_dot, simd_quantized_matmul, simd_quantized_matvec,
    };
    pub use super::quantization::{
        dequantize_matrix, dequantize_vector, fake_quantize, quantize_matrix,
        quantize_matrix_per_channel, quantize_vector, quantized_dot, quantized_matmul,
        quantized_matvec, QuantizationMethod, QuantizationParams, QuantizedDataType,
        QuantizedMatrix, QuantizedVector,
    };
    pub use super::random::{
        banded, diagonal, hilbert, low_rank, normal, orthogonal, permutation, random_correlation,
        sparse, spd, toeplitz, uniform, vandermonde, with_condition_number, with_eigenvalues,
    };
    pub use super::random_matrices::{
        random_complex_matrix, random_hermitian, random_matrix, Distribution1D, MatrixType,
    };
    // 一時的にrandom_newエクスポートを無効化（コンパイル問題解決まで）
    // pub use super::random_new::{
    //     uniform as enhanced_uniform, normal as enhanced_normal, complex as complex_random,
    //     orthogonal as enhanced_orthogonal, unitary, hilbert as enhanced_hilbert,
    //     toeplitz as enhanced_toeplitz, vandermonde as enhanced_vandermonde
    // };
    pub use super::generic::{
        gdet, geig, gemm, gemv, ginv, gnorm, gqr, gsolve, gsvd, GenericEigen, GenericQR,
        GenericSVD, LinalgScalar, PrecisionSelector,
    };
    #[cfg(feature = "simd")]
    pub use super::simd_ops::{
        simd_axpy_f32, simd_axpy_f64, simd_dot_f32, simd_dot_f64, simd_frobenius_norm_f32,
        simd_frobenius_norm_f64, simd_matmul_f32, simd_matmul_f64, simd_matrix_max_f32,
        simd_matrix_max_f64, simd_matrix_min_f32, simd_matrix_min_f64, simd_matvec_f32,
        simd_matvec_f64,
    };
    pub use super::solve::{lstsq, solve, solve_multiple, solve_triangular};
    pub use super::sparse_dense::{
        dense_sparse_matmul, dense_sparse_matvec, sparse_dense_add, sparse_dense_elementwise_mul,
        sparse_dense_matmul, sparse_dense_matvec, sparse_dense_sub, sparse_from_ndarray,
        SparseMatrixView,
    };
    pub use super::special::block_diag;
    pub use super::specialized::{
        specialized_to_operator, BandedMatrix, SpecializedMatrix, SymmetricMatrix,
        TridiagonalMatrix,
    };
    pub use super::stats::{correlation_matrix, covariance_matrix};
    pub use super::structured::{
        solve_circulant, solve_toeplitz, structured_to_operator, CirculantMatrix, HankelMatrix,
        StructuredMatrix, ToeplitzMatrix,
    };
    #[cfg(feature = "tensor_contraction")]
    pub use super::tensor_contraction::{batch_matmul, contract, einsum, hosvd};

    // Automatic differentiation support
    #[cfg(feature = "autograd")]
    pub mod autograd {
        //! Automatic differentiation for linear algebra operations
        //!
        //! Note: The autograd module is currently undergoing a major API redesign.
        //! For basic usage, see examples/autograd_simple_example.rs which demonstrates
        //! how to use scirs2-autograd directly with linear algebra operations.

        // Re-export the module itself for documentation purposes
        pub use super::super::autograd::*;
    }

    // Accelerated implementations
    pub mod accelerated {
        //! Accelerated linear algebra operations using native BLAS/LAPACK
        pub use super::super::blas_accelerated::{
            dot, gemm, gemv, inv as fast_inv, matmul, norm, solve as fast_solve,
        };
        pub use super::super::lapack_accelerated::{
            cholesky as fast_cholesky, eig as fast_eig, eigh as fast_eigh, lu as fast_lu,
            qr as fast_qr, svd as fast_svd,
        };
    }
}
