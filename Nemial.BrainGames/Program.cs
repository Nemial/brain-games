using Nemial.BrainGames.Core;
using Nemial.BrainGames.Games;

Console.WriteLine("Добро пожаловать в Brain Games!");
Console.WriteLine("Введите название игры, чтобы начать!");
Console.WriteLine("Доступные игры: even, progression, prime, gcd, calc");

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
        Console.WriteLine("Такой игры не существует!");
        Environment.Exit(1);
        break;
}