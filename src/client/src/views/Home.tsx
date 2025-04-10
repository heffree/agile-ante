import { useState } from "react";

export function Home() {
  const [count, setCount] = useState(0);

  const handleButtonClick = async () => {
    setCount(count + 1);
    const response = await fetch('/increment', {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      }
    });


    if (response.ok) {
      console.log("Request Successful");
    } else {
      console.error("Request failed");
    }
  }

  const createRoom = async () => {
    const response = await fetch('/create-room', {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      }
    })

    if (response.ok) {
      console.log("Request Successful");
    } else {
      console.error("Request failed");
    }
  }

  const getRooms = async () => {
    const response = await fetch('/get-rooms', {
      method: "GET",
      headers: {
        "Content-Type": "application/json"
      }
    })

    if (response.ok) {
      console.log("Request Successful");
    } else {
      console.error("Request failed");
    }
  }

  return (
    <>
      <div className="card">
        <button onClick={getRooms}>
          Get Rooms
        </button>
        <button onClick={createRoom}>
          Create Room
        </button>
        <button onClick={handleButtonClick}>
          count is {count}
        </button>
      </div>
    </>
  )
}

