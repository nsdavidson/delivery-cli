#
# Copyright 2015 YOUR NAME
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# These options are required for all software definitions
name "delivery-cli"

dependency "openssl"

source path: File.expand_path('..', Omnibus::Config.project_root)

build do
  # Setup a default environment from Omnibus - you should use this Omnibus
  # helper everywhere. It will become the default in the future.
  env = with_standard_compiler_flags(with_embedded_path)
  env['DELIV_CLI_GIT_SHA'] = %x(git rev-parse --short HEAD).strip
  env['RUSTC_VERSION'] = %x(rustc --version).strip

  command "cargo build -j #{workers} --release", env: env

  mkdir "#{install_dir}/bin"
  copy "#{project_dir}/target/release/delivery", "#{install_dir}/bin/delivery"
end
