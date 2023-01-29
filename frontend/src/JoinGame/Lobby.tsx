import React from "react";
import { ReactComponent as White } from './Whitetest.svg';
import { BlockPicker, ColorChangeHandler, ColorResult } from 'react-color';
//@ts-ignore
import { LightenDarkenColor } from 'lighten-darken-color';
import start from "./start.png"

type lobbyProps = {username: string}
type lobbyStates = {
    background: string,
    check: boolean,
}

class Lobby extends React.Component<lobbyProps, lobbyStates> {

    constructor(props: lobbyProps) {
        super(props);
        this.state = {
            background: "#000000",
            check: false,
        }
    }



    handleChange = (color: ColorResult) => {
        this.setState(prevState => ({...prevState, background: color.hex, }));
        document.documentElement.style.setProperty('--base-color', color.hex);
        document.documentElement.style.setProperty('--shadow-color', LightenDarkenColor(color.hex, -35));
    }
    myfunction() {
        console.log("CLICKED");
    }
    render() {
        return (
            <div className='lobby-background'>
                <div className='player-box center'>
                    <h3 className = 'player-text'>{this.props.username}</h3>
                    <White className="player" onClick={() => this.setState(prevState => ({...prevState, check: !prevState.check,}))} />
                    <button style={{ display: this.state.check ? "none" : "initial" }} className = "start-button"><img src= {start} onClick={this.myfunction} /></button>
                </div>
                <div style={{ display: this.state.check ? "initial" : "none" }} className={'colorPick center'}>
                    <BlockPicker color = {this.state.background} onChange={this.handleChange} />
                </div>
            </div>
        );
    };
}


export default Lobby;