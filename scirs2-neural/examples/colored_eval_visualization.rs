use ndarray::{Array, Array1, Array2};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scirs2_neural::error::Result;
use scirs2_neural::utils::{
    colorize, stylize, Color, ColorOptions, ConfusionMatrix, FeatureImportance, LearningCurve,
    ROCCurve, Style,
};

fn main() -> Result<()> {
    println!(
        "{}",
        stylize("Neural Network Model Evaluation with Color", Style::Bold)
    );
    println!("{}", "-".repeat(50));

    // Set up color options
    let color_options = ColorOptions {
        enabled: true,
        use_background: false,
        use_bright: true,
    };

    // Generate some example data
    let n_samples = 500;
    let n_features = 10;
    let n_classes = 4;

    println!(
        "\n{} {} {} {} {} {}",
        colorize("Generating", Color::BrightGreen),
        colorize(n_samples.to_string(), Color::BrightYellow),
        colorize("samples with", Color::BrightGreen),
        colorize(n_features.to_string(), Color::BrightYellow),
        colorize("features for", Color::BrightGreen),
        colorize(n_classes.to_string(), Color::BrightYellow),
    );

    // Create a deterministic RNG for reproducibility
    let mut rng = SmallRng::seed_from_u64(42);

    // 1. Confusion Matrix Example
    println!(
        "\n{}",
        stylize("1. CONFUSION MATRIX VISUALIZATION", Style::Bold)
    );

    // Generate random predictions and true labels
    let y_true = Array::from_shape_fn(n_samples, |_| rng.random_range(0..n_classes));

    // Create slightly correlated predictions (not completely random)
    let y_pred = Array::from_shape_fn(n_samples, |i| {
        if rng.random::<f32>() < 0.7 {
            // 70% chance of correct prediction
            y_true[i]
        } else {
            // 30% chance of random class
            rng.random_range(0..n_classes)
        }
    });

    // Create confusion matrix
    let class_labels = vec![
        "Class A".to_string(),
        "Class B".to_string(),
        "Class C".to_string(),
        "Class D".to_string(),
    ];

    let cm = ConfusionMatrix::<f32>::new(
        &y_true.view(),
        &y_pred.view(),
        Some(n_classes),
        Some(class_labels),
    )?;

    // Print raw and normalized confusion matrices with color
    println!("\n{}", colorize("Raw Confusion Matrix:", Color::BrightCyan));
    println!(
        "{}",
        cm.to_ascii_with_options(Some("Confusion Matrix"), false, &color_options)
    );

    println!(
        "\n{}",
        colorize("Normalized Confusion Matrix:", Color::BrightCyan)
    );
    println!(
        "{}",
        cm.to_ascii_with_options(Some("Normalized Confusion Matrix"), true, &color_options)
    );

    // Print metrics
    println!(
        "\n{} {:.3}",
        colorize("Overall Accuracy:", Color::BrightMagenta),
        cm.accuracy()
    );

    let precision = cm.precision();
    let recall = cm.recall();
    let f1 = cm.f1_score();

    println!("{}", colorize("Per-class metrics:", Color::BrightMagenta));
    for i in 0..n_classes {
        println!(
            "  {}: {}={:.3}, {}={:.3}, {}={:.3}",
            colorize(format!("Class {}", i), Color::BrightYellow),
            colorize("Precision", Color::BrightCyan),
            precision[i],
            colorize("Recall", Color::BrightGreen),
            recall[i],
            colorize("F1", Color::BrightBlue),
            f1[i]
        );
    }

    println!(
        "{} {:.3}",
        colorize("Macro F1 Score:", Color::BrightMagenta),
        cm.macro_f1()
    );

    // 2. Feature Importance Visualization
    println!(
        "\n{}",
        stylize("2. FEATURE IMPORTANCE VISUALIZATION", Style::Bold)
    );

    // Generate random feature importance scores
    let feature_names = (0..n_features)
        .map(|i| format!("Feature_{}", i))
        .collect::<Vec<String>>();

    let importance = Array1::from_shape_fn(n_features, |i| {
        // Make some features more important than others
        let base = (n_features - i) as f32 / n_features as f32;
        base + 0.2 * rng.random::<f32>()
    });

    let fi = FeatureImportance::new(feature_names, importance)?;

    // Print full feature importance with color
    println!(
        "{}",
        fi.to_ascii_with_options(Some("Feature Importance"), 60, None, &color_options)
    );

    // Print top-5 features with color
    println!(
        "\n{}",
        colorize("Top 5 Most Important Features:", Color::BrightCyan)
    );
    println!(
        "{}",
        fi.to_ascii_with_options(Some("Top 5 Features"), 60, Some(5), &color_options)
    );

    // 3. ROC Curve for Binary Classification
    println!("\n{}", stylize("3. ROC CURVE VISUALIZATION", Style::Bold));

    // Generate binary classification data
    let n_binary = 200;
    let y_true_binary = Array::from_shape_fn(n_binary, |_| rng.random_range(0..2));

    // Generate scores with some predictive power
    let y_scores = Array1::from_shape_fn(n_binary, |i| {
        if y_true_binary[i] == 1 {
            // Higher scores for positive class
            0.6 + 0.4 * rng.random::<f32>()
        } else {
            // Lower scores for negative class
            0.4 * rng.random::<f32>()
        }
    });

    let roc = ROCCurve::new(&y_true_binary.view(), &y_scores.view())?;

    println!(
        "{} {:.3}",
        colorize("ROC AUC:", Color::BrightMagenta),
        roc.auc
    );

    println!("\n{}", roc.to_ascii(None, 50, 20));

    // 4. Learning Curve Visualization
    println!(
        "\n{}",
        stylize("4. LEARNING CURVE VISUALIZATION", Style::Bold)
    );

    // Generate learning curve data
    let n_points = 10;
    let n_cv = 5;

    let train_sizes = Array1::from_shape_fn(n_points, |i| 50 + i * 50);

    // Generate training scores (decreasing with size due to overfitting)
    let train_scores = Array2::from_shape_fn((n_points, n_cv), |(i, _)| {
        0.95 - 0.05 * (i as f32 / n_points as f32) + 0.03 * rng.random::<f32>()
    });

    // Generate validation scores (increasing with size)
    let val_scores = Array2::from_shape_fn((n_points, n_cv), |(i, _)| {
        0.7 + 0.2 * (i as f32 / n_points as f32) + 0.05 * rng.random::<f32>()
    });

    let lc = LearningCurve::new(train_sizes, train_scores, val_scores)?;

    println!("{}", lc.to_ascii(None, 60, 20, "Accuracy"));

    // Print final message with color
    println!(
        "\n{}",
        colorize(
            "Model evaluation visualizations completed successfully!",
            Color::BrightGreen
        )
    );

    Ok(())
}
