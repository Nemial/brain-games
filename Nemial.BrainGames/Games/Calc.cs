using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Calc : IGame
{
    public void Run()
    {
        Console.WriteLine("Добро пожаловать в Brain Calc!");
        Console.WriteLine("Вычислите выражение");

        string[] operations = ["+", "-", "*"];

        var random = new Random();
        var firstNum = random.Next(1, 512);
        var secondNum = random.Next(firstNum);
        var operationIndex = random.Next(operations.Length);
        var operation = operations[operationIndex];

        var expressionResult = GetExpressionResult(firstNum, secondNum, operation);


        Console.WriteLine($"{firstNum} {operation} {secondNum}");

        var userInput = Console.ReadLine();
        var userAnswer = int.Parse(userInput?.Trim().ToLower() ?? string.Empty);

        if (expressionResult == userAnswer)
        {
            Console.WriteLine("Вы выиграли");
        }
        else
        {
            Console.WriteLine("Вы проиграли!");
            Console.WriteLine("Правильный ответ " + expressionResult);
        }
    }

    private static int GetExpressionResult(int firstOperand, int secondOperand, string operation)
    {
        switch (operation)
        {
            case "+":
                return firstOperand + secondOperand;
            case "-":
                return firstOperand - secondOperand;
            case "*":
                return firstOperand * secondOperand;
            default:
                throw new ArgumentException($"Unknown operation: {operation}");
        }
    }
}