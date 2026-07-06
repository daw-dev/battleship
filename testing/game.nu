def main [host: string = "dawspie", --fast] {
# "me" boats:
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  ~ | X | ~ | X | X | X | ~ | ~
#  ~ | X | ~ | ~ | ~ | ~ | X | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | X | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | X | ~
#  ~ | ~ | X | X | ~ | ~ | X | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
  let me = { id: "me" }

  let my_boats = [
    {
      starting_position: [1, 6],
      direction: "South",
      len: 2,
    },
    {
      starting_position: [3, 6],
      direction: "East",
      len: 3,
    },
    {
      starting_position: [6, 5],
      direction: "South",
      len: 4,
    },
    {
      starting_position: [2, 2],
      direction: "East",
      len: 2,
    }
  ]

# "other" boats:
#  X | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  X | ~ | ~ | ~ | ~ | ~ | ~ | X
#  X | ~ | X | X | ~ | ~ | ~ | X
#  X | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
#  ~ | ~ | ~ | ~ | X | X | X | ~
#  ~ | ~ | ~ | ~ | ~ | ~ | ~ | ~
  let other = { id: "other" }
  
  let others_boats = [
    {
      starting_position: [0, 7],
      direction: "South",
      len: 4,
    },
    {
      starting_position: [4, 1],
      direction: "East",
      len: 3,
    },
    {
      starting_position: [2, 5],
      direction: "East",
      len: 2,
    },
    {
      starting_position: [7, 6],
      direction: "South",
      len: 2,
    }
  ]

  print "Registering 'me'..."
  let my_game = mosquitto_rr -h $host -t "battleship/register" -e $"battleship/($me.id)/assign" -m ($me | to json) | from json

  print "My Game Assigned:"
  print $my_game

  print "Registering 'other'..."
  let others_game = mosquitto_rr -h $host -t "battleship/register" -e $"battleship/($other.id)/assign" -m ($other | to json) | from json

  print "Other Game Assigned:"
  print $others_game

  let my_action = $"battleship/game/($my_game.game_id)/($my_game.role)/action"
  let my_event = $"battleship/game/($my_game.game_id)/($my_game.role)/event"

  let others_action = $"battleship/game/($others_game.game_id)/($others_game.role)/action"
  let others_event = $"battleship/game/($others_game.game_id)/($others_game.role)/event"

  let state_topic = $"battleship/game/($my_game.game_id)/state"
  let log_file = "state.log"
  if ($log_file | path exists) { rm $log_file }

  # Start background subscriber for game state
  bash -c $"mosquitto_sub -h ($host) -t ($state_topic) > ($log_file) 2>/dev/null &"

  # Sleep a bit to ensure subscription is active
  sleep 500ms

  print "Sending Setup Data..."
  mosquitto_pub -h $host -t $my_action -m ({ Setup: $my_boats } | to json)
  mosquitto_pub -h $host -t $others_action -m ({ Setup: $others_boats } | to json)

  # Sleep to ensure the setup completes and turns start
  sleep 500ms

  # Predefined lists of target coordinates containing both hits and misses
  let host_targets = [
    [0, 0], # Miss
    [0, 7], [0, 6], [0, 5], [0, 4], # Hits
    [5, 5], # Miss
    [4, 1], [5, 1], [6, 1], # Hits
    [7, 7], # Miss
    [2, 5], [3, 5], # Hits
    [0, 1], # Miss
    [7, 6], [7, 5] # Hits
  ]

  let guest_targets = [
    [0, 0], # Miss
    [1, 6], [1, 5], # Hits
    [5, 5], # Miss
    [3, 6], [4, 6], [5, 6], # Hits
    [7, 7], # Miss
    [6, 5], [6, 4], [6, 3], [6, 2], # Hits
    [0, 1], # Miss
    [2, 2], [3, 2] # Hits
  ]

  mut host_idx = 0
  mut guest_idx = 0
  mut turn = "guest" # Guest starts first

  loop {
    # Check if there is a winner in the log file
    if ($log_file | path exists) {
      let content = open $log_file
      if ($content =~ "winner") {
        print "Game Over detected from server state!"
        print $content
        break
      }
    }

    if $turn == "guest" {
      if $guest_idx >= ($guest_targets | length) {
        print "Guest has no more targets, but game is not over?"
        break
      }
      let target = $guest_targets | get $guest_idx
      $guest_idx = $guest_idx + 1

      print $"Guest's turn: Shooting ($target)..."
      let hit = mosquitto_rr -h $host -t $others_action -e $others_event -m ({ Shoot: $target } | to json) | from json
      print $hit

      $turn = "host"
    } else {
      if $host_idx >= ($host_targets | length) {
        print "Host has no more targets, but game is not over?"
        break
      }
      let target = $host_targets | get $host_idx
      $host_idx = $host_idx + 1

      print $"Host's turn: Shooting ($target)..."
      let hit = mosquitto_rr -h $host -t $my_action -e $my_event -m ({ Shoot: $target } | to json) | from json
      print $hit

      $turn = "guest"
    }

    if not $fast {
      sleep 1sec
    }
  }

  # Clean up background process
  bash -c $"pkill -f 'mosquitto_sub -h ($host) -t ($state_topic)'"
  if ($log_file | path exists) { rm $log_file }

  print "Finished!"
}
