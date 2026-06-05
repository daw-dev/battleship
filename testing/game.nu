def main [host: string = "127.0.0.1"] {
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

  print "Sending Setup Data..."
  mosquitto_pub -h $host -t $my_action -m ({ Setup: $my_boats } | to json)
  mosquitto_pub -h $host -t $others_action -m ({ Setup: $others_boats } | to json)

  let hit = mosquitto_rr -h $host -t $others_action -e $others_event -m ({ Shoot: [2, 2] } | to json) | from json

  print "Others turn:"
  print $hit

  print "Finished!"
}
