//! RMSprop optimizer implementation

use crate::error::{NeuralError, Result};
use crate::optimizers::Optimizer;
use ndarray::{Array, ScalarOperand};
use num_traits::Float;
use std::fmt::Debug;

/// RMSprop optimizer
///
/// Implements the RMSprop algorithm proposed by Geoff Hinton in his
/// Coursera class. RMSprop is an adaptive learning rate method that
/// maintains a moving average of squared gradients for each parameter.
///
/// Formula:
/// v_t = rho * v_{t-1} + (1 - rho) * g_t^2
/// param_t = param_{t-1} - learning_rate * g_t / (sqrt(v_t) + epsilon)
///
/// # Examples
///
/// ```
/// use ndarray::Array1;
/// use scirs2_neural::optimizers::{RMSprop, Optimizer};
///
/// // Create a simple RMSprop optimizer
/// let mut rmsprop = RMSprop::<f64>::new(0.001);
///
/// // or with custom decay rate and epsilon
/// let mut rmsprop_custom = RMSprop::new_with_config(0.001, 0.9, 1e-8, 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct RMSprop<F: Float + ScalarOperand + Debug> {
    /// Learning rate
    learning_rate: F,
    /// Decay rate for the moving average of squared gradients
    rho: F,
    /// Small constant for numerical stability
    epsilon: F,
    /// Weight decay factor (L2 regularization)
    weight_decay: F,
    /// Moving average of squared gradients for each parameter array
    v: Vec<Array<F, ndarray::IxDyn>>,
}

impl<F: Float + ScalarOperand + Debug> RMSprop<F> {
    /// Creates a new RMSprop optimizer with the given learning rate and default parameters
    ///
    /// # Arguments
    ///
    /// * `learning_rate` - The learning rate for parameter updates
    pub fn new(learning_rate: F) -> Self {
        let rho = F::from(0.9).unwrap_or_else(|| {
            panic!("Failed to convert 0.9 to the appropriate floating point type")
        });
        let epsilon = F::from(1e-8).unwrap_or_else(|| {
            panic!("Failed to convert 1e-8 to the appropriate floating point type")
        });

        Self {
            learning_rate,
            rho,
            epsilon,
            weight_decay: F::zero(),
            v: Vec::new(),
        }
    }

    /// Creates a new RMSprop optimizer with the full configuration
    ///
    /// # Arguments
    ///
    /// * `learning_rate` - The learning rate for parameter updates
    /// * `rho` - Decay rate for the moving average of squared gradients
    /// * `epsilon` - Small constant for numerical stability
    /// * `weight_decay` - Weight decay factor (L2 regularization)
    pub fn new_with_config(learning_rate: F, rho: F, epsilon: F, weight_decay: F) -> Self {
        Self {
            learning_rate,
            rho,
            epsilon,
            weight_decay,
            v: Vec::new(),
        }
    }

    /// Gets the rho parameter (decay rate)
    pub fn get_rho(&self) -> F {
        self.rho
    }

    /// Sets the rho parameter (decay rate)
    pub fn set_rho(&mut self, rho: F) -> &mut Self {
        self.rho = rho;
        self
    }

    /// Gets the epsilon parameter
    pub fn get_epsilon(&self) -> F {
        self.epsilon
    }

    /// Sets the epsilon parameter
    pub fn set_epsilon(&mut self, epsilon: F) -> &mut Self {
        self.epsilon = epsilon;
        self
    }

    /// Gets the weight decay parameter
    pub fn get_weight_decay(&self) -> F {
        self.weight_decay
    }

    /// Sets the weight decay parameter
    pub fn set_weight_decay(&mut self, weight_decay: F) -> &mut Self {
        self.weight_decay = weight_decay;
        self
    }

    /// Resets the internal state of the optimizer
    pub fn reset(&mut self) {
        self.v.clear();
    }
}

impl<F: Float + ScalarOperand + Debug> Optimizer<F> for RMSprop<F> {
    fn update(
        &mut self,
        params: &mut [Array<F, ndarray::IxDyn>],
        grads: &[Array<F, ndarray::IxDyn>],
    ) -> Result<()> {
        if params.len() != grads.len() {
            return Err(NeuralError::TrainingError(format!(
                "Number of parameter arrays ({}) does not match number of gradient arrays ({})",
                params.len(),
                grads.len()
            )));
        }

        // Initialize moving average if needed
        if self.v.len() != params.len() {
            self.v = params.iter().map(|p| Array::zeros(p.raw_dim())).collect();
        }

        let one_minus_rho = F::one() - self.rho;

        // Update parameters for each param-grad pair
        for i in 0..params.len() {
            // Apply weight decay to gradients if needed
            let adjusted_grad = if self.weight_decay > F::zero() {
                &grads[i] + &(&params[i] * self.weight_decay)
            } else {
                grads[i].clone()
            };

            // Update moving average of squared gradients
            self.v[i] = &self.v[i] * self.rho + &(adjusted_grad.mapv(|x| x * x) * one_minus_rho);

            // Compute parameter update
            let denom = self.v[i].mapv(|x| x.sqrt()) + self.epsilon;
            params[i] = &params[i] - &(&adjusted_grad / denom * self.learning_rate);
        }

        Ok(())
    }

    fn get_learning_rate(&self) -> F {
        self.learning_rate
    }

    fn set_learning_rate(&mut self, lr: F) {
        self.learning_rate = lr;
    }
}
