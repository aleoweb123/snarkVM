// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> Stack<N> {
    /// Authorizes a call to the program function for the given inputs.
    #[inline]
    pub fn authorize<A: circuit::Aleo<Network = N>, R: Rng + CryptoRng>(
        &self,
        private_key: &PrivateKey<N>,
        function_name: impl TryInto<Identifier<N>>,
        inputs: impl ExactSizeIterator<Item = impl TryInto<Value<N>>>,
        rng: &mut R,
    ) -> Result<Authorization<N>> {
        let timer = timer!("Stack::authorize");

        let authorize = "[Stack] [authorize] all time";
        web_sys::console::time_with_label(authorize);

        let logname = "[Stack] [authorize] Verify the number of inputs";
        web_sys::console::log_1(&logname.into());
        web_sys::console::time_with_label(logname);
        
        // Ensure the program contains functions.
        ensure!(!self.program.functions().is_empty(), "Program '{}' has no functions", self.program.id());

        // Prepare the function name.
        let function_name = function_name.try_into().map_err(|_| anyhow!("Invalid function name"))?;
        // Retrieve the function.
        let function = self.get_function(&function_name)?;
        // Retrieve the input types.
        let input_types = function.input_types();
        // Ensure the number of inputs matches the number of input types.
        if function.inputs().len() != input_types.len() {
            bail!(
                "Function '{function_name}' in program '{}' expects {} inputs, but {} types were found.",
                self.program.id(),
                function.inputs().len(),
                input_types.len()
            )
        }
        lap!(timer, "Verify the number of inputs");

        let logname = "[Stack] [authorize] Compute the request";
        web_sys::console::log_1(&logname.into());
        web_sys::console::time_with_label(logname);
        // Compute the request.
        let request = Request::sign(private_key, *self.program.id(), function_name, inputs, &input_types, rng)?;
        lap!(timer, "Compute the request");

        let logname = "[Stack] [authorize] Construct the authorization from the function";
        web_sys::console::log_1(&logname.into());
        web_sys::console::time_with_label(logname);
        // Initialize the authorization.
        let authorization = Authorization::new(&[request.clone()]);
        // Construct the call stack.
        let call_stack = CallStack::Authorize(vec![request], *private_key, authorization.clone());
        // Construct the authorization from the function.
        let _response = self.execute_function::<A>(call_stack)?;
        web_sys::console::time_end_with_label(logname);
        lap!(timer, "Construct the authorization from the function");

        web_sys::console::time_end_with_label(authorize);
        finish!(timer);

        // Return the authorization.
        Ok(authorization)
    }
}
