import React from "react";


class Crewmate extends React.Component {
    render () {
        return (
            <div className = 'background flex'>
                <div className = 'flex'>
                <img src='https://media.tenor.com/qCqLC7df-eIAAAAC/among-us-shhhhhhh.gif'/>
                    <label className = 'user-label' htmlFor="name">Username:</label>
                </div>
            </div>
        );
        }   
}

export default Crewmate;