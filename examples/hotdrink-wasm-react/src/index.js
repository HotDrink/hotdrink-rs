import React from 'react';
import ReactDOM from 'react-dom';

const wasm = window.wasm_bindgen;

class RunWasm extends React.Component {
  constructor(props) {
    super(props);
    let cs = wasm.sum();
    cs.listen(e => cs.notify(e.data));
    cs.subscribe("Sum", "a", v => {
      console.log("a =", v);
      this.setState({ a: v })
    });
    cs.subscribe("Sum", "b", v => {
      console.log("b =", v);
      this.setState({ b: v })
    });
    cs.subscribe("Sum", "c", v => {
      console.log("c =", v);
      this.setState({ c: v })
    });
    cs.update();
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
      console.log(`Set ${variable} to ${value}`);
      if (variable === "a") {
        this.setState({ a: value });
      } else if (variable === "b") {
        this.setState({ b: value });
      } else if (variable === "c") {
        this.setState({ c: value });
      }
      this.state.cs.set_variable("Sum", variable, wasm.I32Wrapper.i32(value));
      this.state.cs.update();
    }
  }

  render() {
    return (
      <div>
        <h1>Sum</h1>
        <input type="number" onChange={this.handleChange("a")} placeholder="a" value={this.state.a}></input>
        +
        <input type="number" onChange={this.handleChange("b")} placeholder="b" value={this.state.b}></input>
        =
        <input type="number" onChange={this.handleChange("c")} placeholder="c" value={this.state.c}></input>
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