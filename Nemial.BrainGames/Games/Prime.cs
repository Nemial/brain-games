using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Prime : IGame
{
    public void Run()
    {
        Console.WriteLine("Добро пожаловать в Brain Prime!");
        Console.WriteLine("Напишите 'да' если число простое, иначе 'нет'");

        var random = new Random();
        var num = random.Next(512);
        var answer = IsPrime(num) ? "да" : "нет";

        Console.WriteLine(num);

        var userInput = Console.ReadLine();
        var userAnswer = userInput?.Trim().ToLower() ?? string.Empty;

        if (userAnswer == answer)
        {
            Console.WriteLine("Вы выиграли!");
        }
        else
        {
            Console.WriteLine("Вы проиграли!");
            Console.WriteLine("Правильный ответ: " + answer);
        }
    }

    private static bool IsPrime(int number)
    {
        if (number < 2)
        {
            return false;
        }

        for (var i = 2; i < number; i += 1)
        {
            if (number % i == 0)
            {
                return false;
            }
        }

        return true;
    }
}