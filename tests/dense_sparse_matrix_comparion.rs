use matrixcompare::compare_matrices;
use matrixcompare::comparators::ExactElementwiseComparator;
use matrixcompare_mock::{dense_matrix_strategy_i64, sparse_matrix_strategy_i64,
                         dense_matrix_strategy_normal_f64, sparse_matrix_strategy_normal_f64};
use proptest::prelude::*;

mod common;
use common::MATRIX_DIM_RANGE;

proptest! {
    #[test]
    fn sparse_and_dense_matrices_should_compare_the_same_as_dense_dense_i64(
        dense in dense_matrix_strategy_i64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE),
        sparse in sparse_matrix_strategy_i64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE)
    ) {
        let c = ExactElementwiseComparator;
        prop_assert_eq!(compare_matrices(&dense, &sparse, &c),
                        compare_matrices(&dense, sparse.to_dense().unwrap(), &c));
    }

    #[test]
    fn sparse_and_dense_matrices_should_compare_the_same_as_dense_dense_f64(
        dense in dense_matrix_strategy_normal_f64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE),
        sparse in sparse_matrix_strategy_normal_f64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE)
    ) {
        let c = ExactElementwiseComparator;
        prop_assert_eq!(compare_matrices(&dense, &sparse, &c),
                        compare_matrices(&dense, sparse.to_dense().unwrap(), &c));
    }

    #[test]
    fn sparse_dense_comparison_is_symmetric_for_all_matrices_i64(
        dense in dense_matrix_strategy_i64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE),
        sparse in sparse_matrix_strategy_i64(MATRIX_DIM_RANGE, MATRIX_DIM_RANGE)
    ) {
        let c = ExactElementwiseComparator;
        let result1 = compare_matrices(&dense, &sparse, &c);
        let result2 = compare_matrices(&sparse, &dense, &c);

        prop_assert_eq!(result1.clone(), result2.clone().reverse());
        prop_assert_eq!(result1.reverse(), result2);
    }
}