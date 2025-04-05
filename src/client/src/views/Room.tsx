import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export function Room() {
  const { id } = useParams();
  const [players, setPlayers] = useState([] as string[]);
  const [playerCount, setPlayerCount] = useState(0);

  const handleButtonClick = async () => {
    const response = await fetch(`/rooms/${id}/players`);
    const result = await response.json();
    setPlayers(result.players);
    setPlayerCount(result.players.length);
  }

  useEffect(() => {
    const eventSource = new EventSource(`/room-connection/${id}`);

    eventSource.onopen = () => {
      fetchData();
    }

    eventSource.onmessage = (event) => {
      const data = JSON.parse(event.data);
      console.log('data', data);
      if (data.command == 'PlayerJoined') {
        setPlayers((players) => [...players, data.value]);
        setPlayerCount((playerCount) => playerCount + 1);
      } else if (data.command == 'PlayerLeft') {
        setPlayers((players) => players.filter((player) => player !== data.value));
        setPlayerCount((playerCount) => playerCount - 1);
      }
    }

    async function fetchData() {
      try {
        const response = await fetch(`/rooms/${id}/players`);
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        const result = await response.json();
        console.log(result);
        setPlayers(result.players as string[]);
        setPlayerCount(result.players.length)
      } catch (error) {
        console.error('Fetch error:', error);
      }
    }

  }, [id]);


  return (
    <>
      <div className="card">
        Players {players}
      </div>
      <button onClick={handleButtonClick}>
        Get Players
      </button>
      <div className="card">
        Player Count {playerCount}
      </div>
    </>
  )
}

