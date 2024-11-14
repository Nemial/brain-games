using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Gcd : IGame
{
    public void Run()
    {
        Console.WriteLine("Добро пожаловать в Brain GCD!");
        Console.WriteLine("Найдите наибольший общий делитель");

        var random = new Random();
        var firstNum = random.Next(1, 512);
        var secondNum = random.Next(1, firstNum);
        var gcd = FindGcd(firstNum, secondNum);

        Console.WriteLine($"{firstNum} {secondNum}");

        var userInput = Console.ReadLine();
        var userAnswer = int.Parse(userInput?.Trim().ToLower() ?? string.Empty);


        if (userAnswer == gcd)
        {
            Console.WriteLine("Вы выиграли");
        }
        else
        {
            Console.WriteLine("Вы проиграли!");
            Console.WriteLine("Правильный ответ: " + gcd);
        }
    }

    private static int FindGcd(int firstNum, int secondNum)
    {
        return firstNum % secondNum != 0 ? FindGcd(secondNum, firstNum % secondNum) : secondNum;
    }
}