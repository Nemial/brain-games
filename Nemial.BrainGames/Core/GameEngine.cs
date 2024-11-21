namespace Nemial.BrainGames.Core;

public class GameEngine
{
    public void StartGame(string name, string description, string question, string answer)
    {
        Console.WriteLine($"Welcome to {name}");
        Console.WriteLine(description);

        Console.WriteLine(question);
        var userInput = Console.ReadLine();
        var userAnswer = userInput?.ToLower().Trim() ?? string.Empty;

        if (userAnswer == answer)
        {
            Console.WriteLine("You won!");
        }
        else
        {
            Console.WriteLine("You lost!");
            Console.WriteLine("Correct answer was: " + answer);
        }
    }
}