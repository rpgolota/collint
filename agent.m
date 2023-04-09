classdef agent < handle
% agent class contains 

    properties % hold data contained in class (state of object)
       id
       guess
       cost 
    end
    
    methods % operations you can carry out on object   
        % Construct an instance of class
        function obj = agent(ID)
            % Assign nonrepeating random numbers from 0-9 to the 10 letters in puzzle 
            j = randperm(10, 10)-1; 
            
            stringX=num2str(j(1,:));
            stringX=stringX(stringX~=' '); % remove the space 
            x=str2double(stringX);
            
            obj.guess = x;
            obj.id = ID;
            obj.cost = abs(obj.guess - 7239056481);    
                
        end
        
        function newGuess = swap(obj, swapIndex1, swapIndex2)
            k1 = floor(obj.guess / 10^(10 - swapIndex1));
            k1 = mod(k1, 10);
            
            k2 = floor(obj.guess / 10^(10 - swapIndex2));
            k2 = mod(k2, 10);
            
            newGuess = obj.guess - k1*10^(10-swapIndex1) - k2*10^(10-swapIndex2)...
                + k2*10^(10-swapIndex1) + k1*10^(10-swapIndex2);
            
            obj.guess = newGuess;
            obj.cost = abs(obj.guess - 7239056481);
            
        end
        
        function newGuess = imitate(obj, imIndex, bestGuess)         
            k1 = floor(obj.guess / 10^(10 - imIndex)); % digit to remove from agent
            k1 = mod(k1, 10);
            
            k2 = floor(bestGuess / 10^(10-imIndex)); % digit to copy from agent with best guess 
            k2 = mod(k2, 10);
            
            % find where k2 was (and put k1 there instead)
            j = mod(floor(obj.guess ./ 10 .^ (10-(1:10))), 10); % 10 because 10 digit-to-letter assignments
            k2Index = find(j == k2); %returns index
            
            newGuess = obj.guess - k1*10^(10-imIndex) - k2*10^(10-k2Index) ...
                + k2*10^(10-imIndex) + k1*10^(10-k2Index);
            
            obj.guess = newGuess;
            obj.cost = abs(obj.guess - 7239056481);
            
        end 
         
    end
end

% How to initialize object
% x = agent;
% x.guess = ' ';
%         function id = agents(inputID)
%         % constructor
%         id.inputID = inputID;
%         id.guess = #; Set property values 
