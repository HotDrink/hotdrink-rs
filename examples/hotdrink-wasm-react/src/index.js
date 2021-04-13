import React from 'react';
import ReactDOM from 'react-dom';

const wasm = window.wasm_bindgen;

class RunWasm extends React.Component {
  constructor(props) {
    super(props);

    // Set up constraint system
    let cs = wasm.sum();
    cs.listen(e => cs.notify(e.data));
    cs.subscribe("Sum", "a", v => this.setState({ a: v }));
    cs.subscribe("Sum", "b", v => this.setState({ b: v }));
    cs.subscribe("Sum", "c", v => this.setState({ c: v }));
    cs.update();

    // Set initial state
    this.state = {
      cs: cs,
      a: 0,
      b: 1,
      c: 0,
    }
  }

  handleChange(variable) {
    return event => {
      let value = parseInt(event.target.value);
      this.setState({ [variable]: value });
      this.state.cs.set_variable("Sum", variable, wasm.I32Wrapper.i32(value));
      this.state.cs.update();
    }
  }

  render() {
    return (
      <div>
        <h1>Sum</h1>
        <input type="number" value={this.state.a} onChange={this.handleChange("a")}></input>
        +
        <input type="number" value={this.state.b} onChange={this.handleChange("b")} ></input>
        =
        <input type="number" value={this.state.c} onChange={this.handleChange("c")} ></input>
      </div>
    )
  }
}

window.wasm_bindgen("./pkg/hotdrink_wasm_multi_threaded_bg.wasm").then(_ => {
  ReactDOM.render(
    <RunWasm />,
    document.getElementById("root"),
  );
})