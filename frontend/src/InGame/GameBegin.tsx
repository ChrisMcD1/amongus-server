import React from "react";
type homeProps = {
    sendName: Function;
}

class GameBegin extends React.Component <homeProps>{
    state = { username: "" }
    handleChange = (e: any) => {
            this.props.sendName(e.target.value);
    }
    render () {
        return (
            <div className = 'background flex'>
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

export default GameBegin;