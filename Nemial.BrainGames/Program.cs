using Nemial.BrainGames.Core;
using Nemial.BrainGames.Games;

Console.WriteLine("Welcome to Brain Games!");
Console.WriteLine("Enter the name of the game to get started!");
Console.WriteLine("Available games: even, progression, prime, gcd, calc");

var gameName = Console.ReadLine();
var preparedGameName = gameName?.Trim().ToLower();

IGame game;
switch (preparedGameName)
{
    case "even":
        game = new Even();
        game.Run();
        break;
    case "progression":
        game = new Progression();
        game.Run();
        break;
    case "prime":
        game = new Prime();
        game.Run();
        break;
    case "gcd":
        game = new Gcd();
        game.Run();
        break;
    case "calc":
        game = new Calc();
        game.Run();
        break;
    default:
        Console.WriteLine("There is no such game!");
        Environment.Exit(1);
        break;
}