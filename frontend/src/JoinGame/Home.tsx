import React from "react";
import { Link as Link } from "react-router-dom";
type homeProps = {
    sendName: Function;
}

class Home  extends React.Component <homeProps>{
    state = { username: "" }
    handleChange = (e: any) => {
            this.props.sendName(e.target.value);
    }
    render () {
        return (
            <div className = 'background flex'>
                <button className = 'joinButton center'><Link to='/lobby' className = 'among-us-join'>Join Game</Link></button>
                <div className = 'flex'>
                    <label className = 'user-label' htmlFor="name">Username:</label>
                    <input onChange={this.handleChange} className = 'user-input' 
                    type="text" defaultValue = {this.state.username}
                     required minLength= {1} maxLength={10} size={12}></input>
                </div>
            </div>
        );
        }   
}

export default Home;